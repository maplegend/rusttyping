use orbtk::prelude::*;
use crate::attributed_text_renderer::*;
use crate::attributed_text_layout::*;
use crate::attributed_text::attributed_text::*;

widget!(
    /// The `TextBlock` widget is used to draw text. It is not interactive.
    ///
    /// **CSS element:** `text-block`
    AttributedTextBlock {
        /// Sets or shares the text property.
        text: AttributedText,

        /// Sets or shares the foreground property.
        foreground: Foreground,

        /// Sets or share the font size property.
        font_size: FontSize,

        /// Sets or shares the font property.
        font: Font,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for AttributedTextBlock {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("TextBlock")
            .text(vec![])
            .foreground(colors::LINK_WATER_COLOR)
            .font_size(fonts::FONT_SIZE_12)
            .font("Roboto Regular")
    }

    fn render_object(&self) -> Option<Box<dyn RenderObject>> {
        Some(Box::new(AttributedTextRenderObject))
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(AttributedTextLayout::new())
    }
}
