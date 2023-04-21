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

//! A text label.

use crate::Widget;
use core::marker::PhantomData;

#[cfg(feature = "piet")]
use {crate::piet::PietBackend, piet::RenderContext};

pub struct Label {
    _private: (),
}

pub struct LabelState<'a> {
    /// The text to display.
    text: &'a str,
    // TODO: Other text properties
}

impl Label {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

#[cfg(feature = "piet")]
impl<R: RenderContext + ?Sized> Widget<PietBackend<'_, R>> for Label {
    type Persistent<'a> = LabelState<'a>;
    type Immediate<'a> = ();

    fn event(
        &self,
        _persistent: &mut Self::Persistent<'_>,
        _immediate: &mut Self::Immediate<'_>,
        _event: crate::Event,
    ) {
    }

    fn rectangle(&self, persistent: &mut Self::Persistent<'_>) -> crate::Rectangle {
        todo!()
    }

    fn render(
        &self,
        persistent: &Self::Persistent<'_>,
        _immediate: &Self::Immediate<'_>,
        backend: &mut PietBackend<'_, R>,
    ) -> Result<(), piet::Error> {
        todo!()
    }
}
