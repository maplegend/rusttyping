use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

pub struct TextGenerator{
    words: Vec<String>
}

impl TextGenerator{

    pub fn new(contents: &str) -> TextGenerator{
        //let mut file = File::open(path).unwrap();
        //let mut contents = String::new();
        //file.read_to_string(&mut contents).unwrap();

        TextGenerator{ words: contents.split('\n').map(|w|w.to_string()).collect() }
    }

    pub fn generate(&self, chars: &Vec<char>, len: usize) -> Vec<String>{
        let mut res: Vec<String> = vec![];
        for i in 0..len{
            let mut rng = rand::thread_rng();
            let word = rng.gen_range(0, self.words.len());
            res.push(self.words[word].clone());
        }
        res
    }
}