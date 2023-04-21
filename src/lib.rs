/*

`sunder` is free software: you can redistribute it and/or modify it under the terms of one of
the following licenses:

- The GNU Affero General Public License as published by the Free Software Foundation, either version
  3 of the License, or (at your option) any later version.
- The Patron License at https://github.com/notgull/sunder/blob/main/LICENSE-PATRON.md, for
  sponsors and contributors, who can ignore the copyleft provisions of the GNU AGPL for this project.

`sunder` is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General
Public License and the Patron License for more details.

You should have received a copy of the GNU Affero General Public License and the corresponding Patron
License along with `sunder`. If not, see <https://www.gnu.org/licenses/>.

*/

//! Widget rendering and event handling for Rust.
//!
//! GUI interfaces generally have two complex parts: event delivery and incrementalization, and widget
//! rendering and event handling. This crate aims to provide a simple yet powerful interface for the
//! latter parts, in order to allow GUI frameworks to focus on event delivery (which is usually what
//! differentiates the GUI from others).
//!
//! The [`Widget`] trait represents a widget that can be rendered to the screen. It takes two objects:
//! one that represents the persistent state of the widget and one that represents the immediate state.
//! It also has functions that define how the widget reacts to events. The goal is to abstract over
//! the widget drawing itself in a way that can be applied to both retained mode and immediate mode
//! GUI.

#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

#[cfg(feature = "piet")]
pub mod piet;

pub mod widgets;

/// The backend for rendering widgets.
pub trait Backend {
    /// The error type for rendering.
    type Error: fmt::Debug + fmt::Display;

    /// The result of a rendering operation.
    type Output;
}

/// The whole point.
pub trait Widget<B: Backend> {
    /// Persistent state of the widget.
    ///
    /// This part is usually consistent between calls. A retained mode GUI might store this in an
    /// object alongside the widget, while an immediate mode GUI might derive this from the parameters
    /// of whatever type defines the widget.
    type Persistent<'a>: 'a;

    /// Immediate state of the widget.
    ///
    /// This is expected to change between calls, often in response to user input. This might contain
    /// fields like "is the button pressed".
    type Immediate<'a>: Default + 'a;

    /// Get the rectangle that this widget is defined by.
    fn rectangle(&self, persistent: &mut Self::Persistent<'_>) -> Rectangle;

    /// Add to the immediate state using an event.
    fn event(
        &self,
        persistent: &mut Self::Persistent<'_>,
        immediate: &mut Self::Immediate<'_>,
        event: Event,
    );

    /// Render the widget.
    fn render(
        &self,
        persistent: &Self::Persistent<'_>,
        immediate: &Self::Immediate<'_>,
        backend: &mut B,
    ) -> Result<B::Output, B::Error>;
}

/// Events that a widget might care about.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Event {
    /// Where the mouse is, relative to the top left corner of this widget.
    Mouse { x: f64, y: f64 },
}

/// Two dimensional rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rectangle {
    /// The x coordinate of the top left corner.
    pub x: i32,

    /// The y coordinate of the top left corner.
    pub y: i32,

    /// The width of the rectangle.
    pub width: u32,

    /// The height of the rectangle.
    pub height: u32,
}
