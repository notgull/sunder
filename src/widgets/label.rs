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

use crate::{RenderedWidget, Widget};

cfg_piet! {
    use crate::piet::PietBackend;
    use piet::{RenderContext, TextLayout, Text, TextLayoutBuilder};
}

/// A label consisting of text.
///
/// This widget is the atomic unit used to render text.
pub struct Label<'a> {
    /// The text to display.
    text: &'a str,

    /// The maximum width of the label.
    max_width: Option<f64>,
    // TODO: Other text properties
}

cfg_piet! {
    pub struct LabelCachePiet<R: RenderContext + ?Sized> {
        /// Text layout.
        layout: Option<R::TextLayout>,
    }

    impl<R: RenderContext + ?Sized> Default for LabelCachePiet<R> {
        fn default() -> Self {
            Self { layout: None }
        }
    }

    impl<R: RenderContext + ?Sized> LabelCachePiet<R> {
        fn populate(&mut self, label: &Label<'_>, ctx: &mut R) -> Result<(), piet::Error> {
            use alloc::string::ToString;

            if let Some(layout) = &self.layout {
                if layout.text() == label.text {
                    // No need to change anything.
                    return Ok(());
                }
            }

            // Build the text layout.
            let mut layout = ctx
                .text()
                .new_text_layout(label.text.to_string());
            if let Some(max_width) = label.max_width {
                layout = layout.max_width(max_width);
            }

            self.layout = Some(layout.build()?);
            Ok(())
        }
    }
}

impl Widget for Label<'_> {
    type Immediate<'a> = ();

    fn handle_event(&mut self, _immediate: &mut Self::Immediate<'_>, _event: crate::Event) -> bool {
        // We don't care about events.
        false
    }
}

#[cfg(feature = "piet")]
impl<R: RenderContext + ?Sized> RenderedWidget<PietBackend<'_, R>> for Label<'_> {
    type Cache = LabelCachePiet<R>;

    fn rectangle(
        &mut self,
        cache: &mut Self::Cache,
        backend: &mut PietBackend<'_, R>,
    ) -> Result<crate::Size, piet::Error> {
        cache.populate(self, backend.context())?;

        let size = cache.layout.as_ref().unwrap().size();
        Ok(crate::Size {
            width: size.width as u32,
            height: size.height as u32,
        })
    }

    fn render(
        &self,
        _: &(),
        cache: &mut Self::Cache,
        backend: &mut PietBackend<'_, R>,
    ) -> Result<(), piet::Error> {
        cache.populate(self, backend.context())?;
        backend
            .context()
            .draw_text(cache.layout.as_ref().unwrap(), (0.0, 0.0));

        Ok(())
    }
}
