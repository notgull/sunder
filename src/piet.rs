// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
// This file is a part of `sunder`.
//
// `sunder` is free software: you can redistribute it and/or modify it under the terms of
// either:
//
// * GNU Lesser General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
// * Mozilla Public License as published by the Mozilla Foundation, version 2.
//
// `sunder` is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License or the Mozilla Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License and the Mozilla
// Public License along with `sunder`. If not, see <https://www.gnu.org/licenses/> or
// <https://www.mozilla.org/en-US/MPL/2.0/>.

//! A backend for rendering widgets to a `piet` context.

use super::Backend;

use ui_theme::Theme;

/// A backend oriented around a [`piet::RenderContext`].
///
/// [`piet::RenderContext`]: https://docs.rs/piet/latest/piet/trait.RenderContext.html
pub struct PietBackend<'a, C: ?Sized> {
    context: &'a mut C,
    theme: &'a Theme,
}

impl<'a, C: piet::RenderContext + ?Sized> PietBackend<'a, C> {
    /// Create a new backend from a `piet` context.
    pub fn new(context: &'a mut C, theme: &'a Theme) -> Self {
        Self { context, theme }
    }

    /// Get the underlying `piet` context.
    pub fn context(&mut self) -> &mut C {
        self.context
    }

    /// Get the underlying theme.
    pub fn theme(&self) -> &Theme {
        self.theme
    }
}

impl<C: piet::RenderContext + ?Sized> Backend for PietBackend<'_, C> {
    type Error = piet::Error;
    type Output = ();
}
