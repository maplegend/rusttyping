extern crate orbtk;
extern crate dces;
use std::cell::{Cell, RefCell};
use std::time::{Duration, Instant};

use orbtk::{
    prelude::*,
    shell::{Key, KeyEvent},
};

use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

//mod attributed_text_layout;
mod attributed_text;
use crate::attributed_text::*;
use crate::attributed_text::attributed_text::*;

mod text_generator;
use text_generator::TextGenerator;

//mod attributed_text_block;
use crate::attributed_text_block::*;
//mod attributed_text_renderer;
use crate::attributed_text_renderer::*;

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
    fn get_styled_text(&self) -> Vec<AttributedLetter>{
        self.text.clone().into_inner().iter().map(|kl| AttributedLetter::new(if kl.character == ' ' {'_'} else {kl.character}, (match kl.pressed {
            Pressed::Pressed => "#239B56",
            Pressed::NotPressed => "#E5E7E9",
            Pressed::WrongPressed => "#E74C3C"
        }).into())).collect()
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
                        .0 = String16::from(format!("Error: {}", self.errors.get()));

                    let len = self.text.borrow().len();

                    let cursor = self.cursor.get();
                    let text = self.text.clone().into_inner().clone();
                    let actual_char = text.get(cursor).unwrap_or(&KeyLetter::default()).character.clone();
                    let correct = actual_char == key;

                    if cursor >= text.len()-1{
                        self.generate_text();
                        self.start_time.set(Instant::now());
                        self.cursor.set(0);
                        self.errors.set(0);
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

                    //context.child_by_id("items").unwrap().set(StyledText(self.get_styled_text()));
                    context.child_by_id("main_text").unwrap().set(AttributedText(self.get_styled_text()));
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
        text: AttributedText
    }
);

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        let text_state = self.clone_state();
        let text_len = text_state.text.borrow().len();
        self.name("MainView").text(state.get_styled_text()).child(
                    Stack::create()
                        .child(create_header(context, "Text"))
                        .child(
                            TextBlock::create()
                                .selector(SelectorValue::new().id("speed"))
                                .text("Speed: 0 cpm")
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .build(context),
                        )
                        .child(
                            TextBlock::create()
                                .selector(SelectorValue::new().id("errors"))
                                .text("Errors: 0")
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .build(context),
                        )
                        .child(
                            AttributedTextBlock::create()
                                .selector(SelectorValue::new().id("main_text"))
                                .text(id)
                                .font_size(20.0)
                                .margin((0.0, 8.0, 0.0, 0.0))
                                .build(context),
                        )
                        .build(context)
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

