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
            let mut build_context = context.build_context();

            for i in 0..text.len() {
                let letter = &text[i];
                let character = TextBlock::create()
                    .selector(Selector::from("item").id(&letter.id))
                    .text(letter.character.to_string())
                    .font_size(20.0)
                    .build(&mut build_context);
                build_context.append_child(items_panel, character);
            }
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

        /// Sets or shares the orientation property.
        orientation: Orientation,

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
                            .orientation(id)
                            .build(context),
                    )
                    .build(context),
            )
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}
