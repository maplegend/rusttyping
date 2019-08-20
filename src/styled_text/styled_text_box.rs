use std::cell::Cell;

use orbtk::prelude::*;

use crate::styled_text::*;

#[derive(Default)]
pub struct STextWidgetState;

impl Into<Rc<dyn State>> for STextWidgetState {
    fn into(self) -> Rc<dyn State> {
        Rc::new(self)
    }
}

impl State for STextWidgetState {
    fn update(&self, context: &mut Context<'_>) {
        if let Some(items_panel) = context.entity_of_child("items_panel") {
            context.clear_children_of(items_panel);

            let text = context.widget().clone_or_default::<StyledText>().0;
            let bounds = context.widget().get::<orbtk::api::Bounds>().0;

            let width = 600.0;

            let mut build_context = context.build_context();

            let mut current_stack = Stack::create()
                .selector(SelectorValue::default().clone().id("items_row")).height(20.0)
                .orientation(OrientationValue::Horizontal).build(&mut build_context);
            build_context.append_child(items_panel, current_stack);

            let words = text.iter()
                .fold(vec![vec![]], |mut rs, l| {
                    let len = rs.len();
                    rs[len-1].push(l);
                    if l.character == '_' {rs.push(vec![])};
                    rs
                });
            println!("words len {}", words.len());
            let mut cw = 0.0;
            for word in words{
                cw += word.len() as f64 * 25.0;
                if cw >= width {
                    current_stack = Stack::create()
                        .selector(SelectorValue::default().clone().id("items_row")).height(20.0)
                        .orientation(OrientationValue::Horizontal).build(&mut build_context);
                    build_context.append_child(items_panel, current_stack);
                    cw = 0.0;
                }
                for i in 0..word.len() {
                    let letter = &word[i];
                    let character = TextBlock::create()
                        .selector(Selector::from("item").id(&letter.id))
                        .text(letter.character.to_string())
                        .font_size(20.0);
                        //.build(&mut build_context);
                    //println!("width {}", character.context.widget().get::<orbtk::api::Bounds>().0.width);
                    let character = character.build(&mut build_context);
                    build_context.append_child(current_stack, character);
                }
            }
            /*
            let mut cw = 0.0;
            for i in 0..text.len() {
                cw += 20.0;
                if cw >= width{
                    println!("skip line {} {}", cw, i);
                    current_stack = Stack::create()
                        .selector(SelectorValue::default().clone().id("items_row")).height(20.0)
                        .orientation(OrientationValue::Horizontal).build(&mut build_context);
                    build_context.append_child(items_panel, current_stack);
                    cw = 0.0;
                }
                let letter = &text[i];
                let character = TextBlock::create()
                    .selector(Selector::from("item").id(&letter.id))
                    .text(letter.character.to_string())
                    .font_size(20.0)
                    .build(&mut build_context);

                build_context.append_child(current_stack, character);
            }
            */
        }
    }
}

widget!(
    /// The `ItemsWidget` is a simple no interactive items drawer widget.
    ///
    /// **CSS element:** `items-widget`
    STextWidget<STextWidgetState> {
        /// Sets or shares the background property.
        background: Background,

        /// Sets or shares the border radius property.
        border_radius: BorderRadius,

        /// Sets or shares the border thickness property.
        border_thickness: BorderThickness,

        /// Sets or shares the border brush property.
        border_brush: BorderBrush,

        /// Sets or shares the padding property.
        padding: Padding,

        styled_text: StyledText,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for STextWidget {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        self.name("ItemsWidget")
            .selector("items-widget")
            .background(colors::LYNCH_COLOR)
            .border_radius(2.0)
            .border_thickness(1.0)
            .border_brush(colors::BOMBAY_COLOR)
            .padding(2.0)
            .child(
                Container::create()
                    .background(id)
                    .border_radius(id)
                    .border_thickness(id)
                    .border_brush(id)
                    .padding(id)
                    .child(
                        Stack::create()
                            .selector(SelectorValue::default().clone().id("items_panel"))
                            .orientation(OrientationValue::Vertical)
                            .build(context),
                    )
                    .build(context),
            )
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}
