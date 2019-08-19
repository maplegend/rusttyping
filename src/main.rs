extern crate orbtk;
use std::cell::{Cell, RefCell};

use orbtk::{
    prelude::*,
    shell::{Key, KeyEvent},
};

use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

mod text_generator;
use text_generator::TextGenerator;

mod styled_text;
use crate::styled_text::*;
mod styled_text_box;
use styled_text_box::STextWidget;
use crate::Action::KeyPressed;

#[derive(Debug, Copy, Clone)]
enum Action {
    KeyPressed(char)
}

#[derive(Debug, Copy, Clone)]
enum Pressed {
    Pressed,
    NotPressed,
    WrongPressed
}

impl Default for Pressed{
    fn default() -> Pressed {
        Pressed::NotPressed
    }
}

#[derive(Default, Copy, Clone)]
struct KeyLetter{
    character: char,
    pressed: Pressed
}

impl KeyLetter{
    pub fn new(character: char, pressed: Pressed) -> KeyLetter{
        KeyLetter{character, pressed}
    }
}

pub struct MainViewState {
    text_gen: TextGenerator,
    cursor: Cell<usize>,
    text: RefCell<Vec<KeyLetter>>,
    action: Cell<Option<Action>>,
}

impl Default for MainViewState {
    fn default() -> Self {
        let mut rng = thread_rng();
        let st = MainViewState {
            text_gen: TextGenerator::new(include_str!("../res/words_filtered.txt")),
            cursor: Cell::new(0),
            text: RefCell::new(vec![]),
            action: Cell::new(None),
        };
        st.text.replace(st.text_gen.generate(&vec!['1'], 5).join(" ")
            .chars().map(|c| KeyLetter::new(c, Pressed::NotPressed)).collect());
        st
    }
}

impl MainViewState {
    fn action(&self, action: impl Into<Option<Action>>) {
        self.action.set(action.into());
    }
    fn get_styled_text(&self) -> Vec<Letter>{
        self.text.clone().into_inner().iter().map(|kl| Letter::new(kl.character, (match kl.pressed {
            Pressed::Pressed => "pressed",
            Pressed::NotPressed => "not_pressed",
            Pressed::WrongPressed => "wrong_pressed"
        }).to_string())).collect()
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::KeyPressed(key) => {
                    if let Some(button_count_text) = context.widget().try_get_mut::<Text>() {
                        button_count_text.0 =
                            String16::from(format!("Key pressed: {}", key));

                        let len = self.text.borrow().len();
                        //let text = self.text.borrow();
                        let cursor = self.cursor.get();
                        let actual_char = self.text.clone().into_inner().clone().get(cursor).unwrap_or(&KeyLetter::default()).character.clone();
                        let correct = actual_char == key;
                        println!("correct1");
                        //self.text.borrow_mut().remove(0);
                        //context.child_by_id("items").unwrap().set(Count(len - 1));
                        //if let Some(key) = self.text.into_inner().get(cursor){
                        self.text.borrow_mut()[cursor] = KeyLetter::new(actual_char, if correct { Pressed::Pressed} else {Pressed::WrongPressed});

                        if correct {
                            self.cursor.set(cursor + 1);
                        }

                        context.child_by_id("items").unwrap().set(StyledText(self.get_styled_text()));
                            //text.drain(0..1);
                        //}

                    }
                }
            }

            self.action.set(None);
        }
    }
}

fn create_header(context: &mut BuildContext, text: &str) -> Entity {
    TextBlock::create()
        .text(text)
        .selector(SelectorValue::new().with("text-block").class("h1"))
        .build(context)
}

widget!(
    MainView<MainViewState>: KeyDownHandler {
        count_text: Text,
        text: StyledText
    }
);

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        let text_state = self.clone_state();
        let text_len = text_state.text.borrow().len();
        self.name("MainView").text(state.get_styled_text()).count_text("123").child(
            Grid::create()
                .margin(8.0)
                .columns(
                    Columns::create()
                        .column("Auto")
                        .column(16.0)
                        .column("Auto")
                        .column(16.0)
                        .column("Auto")
                        .build(),
                )
                .child(
                    Stack::create()
                        .attach(GridColumn(2))
                        .child(create_header(context, "Text"))
                        .child(
                            TextBlock::create()
                                .selector(SelectorValue::new().class("body"))
                                .text(id)
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .attach(GridColumn(2))
                                .attach(GridRow(1))
                                .build(context),
                        )
                        .child(
                            STextWidget::create()
                                .selector(Selector::from("items-widget").id("items"))
                                .padding((4.0, 4.0, 4.0, 2.0))
                                .margin((0.0, 8.0, 0.0, 8.0))
                                .border_thickness(BorderThickness::from(0.0))
                                .orientation(OrientationValue::Horizontal)
                                .styled_text(id)
                                .build(context),
                        )
                        .build(context),
                )
                .build(context),
        ).on_key_down(move |event: KeyEvent| -> bool {
            state.action(Action::KeyPressed(event.text.chars().next().unwrap_or_default()));
            true
        })
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("RTyping")
                .position((100.0, 100.0))
                .size(468.0, 730.0)
                .theme(
                    ThemeValue::create()
                        .extension_css(include_str!("../res/style.css"))
                        .build(),
                )

                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}

