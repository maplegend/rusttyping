use std::time::Instant;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;

//#[derive(Serialize, Deserialize)]
struct SampleStatistic{
    start_time: Instant,
    key_timings: HashMap<char, Vec<usize>>,
    key_errors: HashMap<char, usize>,
    length: usize,
    time: usize,
    errors: usize
}

impl SampleStatistic{
    pub fn new(start_time: Instant,
               key_timings: HashMap<char, Vec<usize>>,
               key_errors: HashMap<char, usize>,
               length: usize,
               time: usize,
               errors: usize) -> SampleStatistic{
        SampleStatistic{
            start_time,
            key_timings,
            key_errors,
            length,
            time,
            errors
        }
    }
}

pub struct TypingState{
    pub speed: f64,
    pub errors: usize
}

pub struct TypingStatistic{
    samples: Vec<SampleStatistic>,
    key_timings: HashMap<char, Vec<usize>>,
    key_errors: HashMap<char, usize>,
    start_sample: Instant,
    start_key: Instant,
    key_count: usize,
    errors_count: usize,
    finished: bool,
}

impl TypingStatistic{
    pub fn new() -> Self{
        TypingStatistic{
            samples: vec![],
            key_timings: HashMap::new(),
            key_errors: HashMap::new(),
            start_sample: Instant::now(),
            start_key: Instant::now(),
            key_count: 0,
            errors_count: 0,
            finished: true,
        }
    }

    pub fn get_current_state(&self) -> TypingState{
        TypingState{
            speed: self.key_count as f64 / (self.start_sample.elapsed().as_secs() as f64 / 60.0),
            errors: self.errors_count
        }
    }

    pub fn is_finished(&self) -> bool{
        self.finished
    }

    pub fn start_sample(&mut self){
        self.start_sample = Instant::now();
        self.finished = false;
    }

    pub fn key_pressed(&mut self, key: char, correct: bool){
        let time = self.start_key.elapsed().as_millis();
        if correct {
            let timings = self.key_timings.entry(key).or_insert(vec![]);
            timings.push(time as usize);
            self.key_count += 1;
        } else{
            let errors = self.key_errors.entry(key).or_insert(0);
            *errors += 1;
            self.errors_count += 1;
        }
    }

    pub fn finish_sample(&mut self) {
        self.samples.push(
            SampleStatistic::new(
                self.start_sample,
                self.key_timings.clone(),
                self.key_errors.clone(),
                self.key_count,
                self.start_sample.elapsed().as_micros() as usize,
                self.errors_count
            )
        );
        self.key_timings = HashMap::new();
        self.key_errors = HashMap::new();
        self.start_sample = Instant::now();
        self.start_key = Instant::now();
        self.key_count = 0;
        self.errors_count = 0;
        self.finished = true;
    }
}