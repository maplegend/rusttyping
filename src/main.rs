extern crate orbtk;
use std::cell::{Cell, RefCell};
use std::time::{Duration, Instant};

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
use std::borrow::BorrowMut;

#[derive(Debug, Copy, Clone)]
enum Action {
    KeyPressed(char)
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    errors: Cell<usize>,
    start_time: Cell<Instant>,
}

impl Default for MainViewState {
    fn default() -> Self {
        let mut rng = thread_rng();
        let st = MainViewState {
            text_gen: TextGenerator::new(include_str!("../res/words_filtered.txt")),
            cursor: Cell::new(0),
            text: RefCell::new(vec![]),
            action: Cell::new(None),
            errors: Cell::new(0),
            start_time: Cell::new(Instant::now())
        };
        st.generate_text();
        st
    }
}

impl MainViewState {
    fn generate_text(&self){
        self.text.replace(self.text_gen.generate(&vec!['1'], 5).join(" ")
            .chars().map(|c| KeyLetter::new(c, Pressed::NotPressed)).collect());
    }
    fn action(&self, action: impl Into<Option<Action>>) {
        self.action.set(action.into());
    }
    fn get_styled_text(&self) -> Vec<Letter>{
        self.text.clone().into_inner().iter().map(|kl| Letter::new(if kl.character == ' ' {'_'} else {kl.character}, (match kl.pressed {
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
                    context
                        .child_by_id("speed")
                        .unwrap()
                        .get_mut::<Text>()
                        .0 = String16::from(format!("Speed: {:.1} cpm", self.cursor.get() as f64 / (self.start_time.get().elapsed().as_secs() as f64 / 60.0)));

                    context
                        .child_by_id("errors")
                        .unwrap()
                        .get_mut::<Text>()
                        .0 = String16::from(format!("Speed: {} cpm", self.errors.get()));

                    let len = self.text.borrow().len();

                    let cursor = self.cursor.get();
                    let text = self.text.clone().into_inner().clone();
                    let actual_char = text.get(cursor).unwrap_or(&KeyLetter::default()).character.clone();
                    let correct = actual_char == key;

                    if cursor >= text.len()-1{
                        self.generate_text();
                        self.start_time.set(Instant::now());
                        self.cursor.set(0);
                    } else{
                        if !correct{
                            if text[cursor].pressed == Pressed::NotPressed {
                                self.errors.set(self.errors.get() + 1);
                            }
                        }

                        self.text.borrow_mut()[cursor] =
                            KeyLetter::new(actual_char, if correct { Pressed::Pressed} else {Pressed::WrongPressed});

                        if correct {
                            self.cursor.set(cursor + 1);
                        }
                    }

                    context.child_by_id("items").unwrap().set(StyledText(self.get_styled_text()));

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
        text: StyledText
    }
);

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        let text_state = self.clone_state();
        let text_len = text_state.text.borrow().len();
        self.name("MainView").text(state.get_styled_text()).child(
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
                                .selector(SelectorValue::new().id("speed"))
                                .text("Speed: 0 cpm")
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .attach(GridColumn(2))
                                .attach(GridRow(1))
                                .build(context),
                        )
                        .child(
                            TextBlock::create()
                                .selector(SelectorValue::new().id("errors"))
                                .text("Errors: 0 cpm")
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .attach(GridColumn(2))
                                .attach(GridRow(1))
                                .build(context),
                        )
                        .child(
                            STextWidget::create()
                                .selector(Selector::from("items-widget").id("items"))
                                //.padding((4.0, 4.0, 4.0, 2.0))
                                .border_thickness(BorderThickness::from(0.0))
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

