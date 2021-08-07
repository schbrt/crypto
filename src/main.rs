#![allow(dead_code)]
use std::fs;
use std::env;
mod ciphers;

fn main() {
//    let encrypted = ciphers::caesar::encipher("This sentence uses a bunch of different letters. Something about a quick brown fox and a lazy dog. Who knows. Extra quack jump.", 12);
    let mut args: Vec<String> = env::args().collect();
    let encrypted = args.pop().unwrap();
    let corpus = fs::read_to_string("eng_text_sample.txt").expect("Error reading file.");
    let shift = ciphers::caesar::run_freq_analysis(&encrypted, &corpus);
    let msg = ciphers::caesar::decipher(&encrypted, shift);
    println!("{}", shift);
    println!("{}", msg);
    //    let x = ciphers::jensen_shannon([1.0, 1.0].to_vec(), [1.0, 1.0].to_vec());
}
