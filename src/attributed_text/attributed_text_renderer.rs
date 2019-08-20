use orbtk::prelude::*;
use crate::attributed_text::attributed_text::*;

/// Used to render a text.
pub struct AttributedTextRenderObject;

impl Into<Box<dyn RenderObject>> for AttributedTextRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for AttributedTextRenderObject {
    fn render(&self, context: &mut Context<'_>, global_position: &Point) {
        let (bounds, text, font, font_size) = {
            let widget = context.widget();
            let text = widget.clone::<AttributedText>();

            let txt = {
                if !text.0.is_empty() {
                    text.0.clone()
                } else {
                    vec![]
                }
            };
            (
                widget.get::<Bounds>().0,
                txt,
                widget.get::<Font>().0.clone(),
                widget.get::<FontSize>().0,
            )
        };

        if !text.is_empty() {
            context.render_context_2_d().save();
            context.render_context_2_d().begin_path();

            context.render_context_2_d().set_font_family(font);
            context.render_context_2_d().set_font_size(font_size);

            let words = text.iter()
                .fold(vec![vec![]], |mut rs, l| {
                    let len = rs.len();
                    rs[len-1].push(l);
                    if l.character == '_' || l.character == ' ' {rs.push(vec![])};
                    rs
                });

            let mut line = 1.0;

            let mut str = String::new();
            let mut x = 0.0;

            for w in 0..words.len() {
                let word = &words[w];
                let word_size = context
                    .render_context_2_d()
                    .measure_text(&word.iter()
                        .fold(String::new(), |mut rs, l| {
                            rs.push(l.character);
                            rs
                        }));
                //println!("new x {}", x + word_size.width);
                if x + word_size.width - 1.0 > bounds.width {
                    line += 1.0;
                    x = 0.0;
                    str = String::new();
                }

                for i in 0..word.len() {
                    let letter = word[i].clone();
                    context.render_context_2_d().set_fill_style(letter.color.0);

                    str.push(letter.character);
                    let size = context.render_context_2_d().measure_text(&str);
                    let width = size.width;
                    context.render_context_2_d().fill_text(
                        &letter.character.to_string(),
                        global_position.x + bounds.x + x,
                        global_position.y + bounds.y + (line as f64 + 1.0)*(size.height),
                        None,
                    );
                    x = width;
                }
            }
            context.render_context_2_d().close_path();
            context.render_context_2_d().restore();
        }
    }
}
