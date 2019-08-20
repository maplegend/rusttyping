extern crate orbtk;
extern crate dces;
extern crate serde;
extern crate serde_json;
use std::cell::{Cell, RefCell};

use orbtk::{
    prelude::*,
    shell::KeyEvent,
};

//mod attributed_text_layout;
mod attributed_text;
use crate::attributed_text::*;
use crate::attributed_text::attributed_text::*;

mod text_generator;
use text_generator::TextGenerator;

mod typing_statistic;
use typing_statistic::*;

use crate::attributed_text_block::*;

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
    statistic: RefCell<TypingStatistic>,
    cursor: Cell<usize>,
    text: RefCell<Vec<KeyLetter>>,
    action: Cell<Option<Action>>,
}

impl Default for MainViewState {
    fn default() -> Self {
        let st = MainViewState {
            text_gen: TextGenerator::new(include_str!("../res/words_filtered.txt")),
            statistic: RefCell::new(TypingStatistic::new()),
            cursor: Cell::new(0),
            text: RefCell::new(vec![]),
            action: Cell::new(None),
        };
        st.generate_text();
        st
    }
}

impl MainViewState {
    fn generate_text(&self){
        self.text.replace(self.text_gen.generate(&vec!['1'], 20).join(" ")
            .chars().map(|c| KeyLetter::new(c, Pressed::NotPressed)).collect());
    }
    fn action(&self, action: impl Into<Option<Action>>) {
        self.action.set(action.into());
    }
    fn get_styled_text(&self) -> Vec<AttributedLetter>{
        self.text.clone().into_inner().iter()
            .map(|kl| AttributedLetter::new(
                if kl.character == ' ' {'_'} else {kl.character},
                (match kl.pressed {
                    Pressed::Pressed => "#239B56",
                    Pressed::NotPressed => "#E5E7E9",
                    Pressed::WrongPressed => "#E74C3C"
                }).into())
            ).collect()
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::KeyPressed(key) => {
                    let mut statistic = self.statistic.borrow_mut();
                    if statistic.is_finished(){
                        statistic.start_sample();
                    }

                    let cursor = self.cursor.get();
                    let text = self.text.clone().into_inner().clone();
                    let actual_char = text.get(cursor).unwrap_or(&KeyLetter::default()).character.clone();
                    let correct = actual_char == key;

                    if cursor >= text.len(){
                        self.generate_text();
                        self.cursor.set(0);
                        statistic.finish_sample();
                    } else{
                        if !correct{
                            if text[cursor].pressed == Pressed::NotPressed {
                                statistic.key_pressed(key, false);
                            }
                        }

                        self.text.borrow_mut()[cursor] =
                            KeyLetter::new(actual_char, if correct { Pressed::Pressed} else {Pressed::WrongPressed});

                        if correct {
                            self.cursor.set(cursor + 1);
                            statistic.key_pressed(key, true);
                        }
                    }

                    context.child_by_id("main_text").unwrap().set(AttributedText(self.get_styled_text()));

                    let current_stat = statistic.get_current_state();
                    context
                        .child_by_id("speed")
                        .unwrap()
                        .get_mut::<Text>()
                        .0 = String16::from(format!("Speed: {:.1} cpm", current_stat.speed.min(1000.0)));

                    context
                        .child_by_id("errors")
                        .unwrap()
                        .get_mut::<Text>()
                        .0 = String16::from(format!("Error: {}",  current_stat.errors));
                }
            }

            self.action.set(None);
        }
    }
}
widget!(
    MainView<MainViewState>: KeyDownHandler {
        text: AttributedText
    }
);

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        let state = self.clone_state();
        self.name("MainView").text(state.get_styled_text()).child(
                    Stack::create()
                        .margin((10.0, 10.0, 10.0, 10.0))
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
                .size(730.0, 400.0)
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

