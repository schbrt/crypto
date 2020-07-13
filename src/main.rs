use std::fs;
mod ciphers;

fn main() {
    let encrypted = ciphers::caesar::encipher("This sentence uses a bunch of different letters. Something about a quick brown fox and a lazy dog. Who knows. Extra quack jump.", 12);
    println!("{}", encrypted);
    let corpus = fs::read_to_string("eng_text_sample.txt").expect("Error reading file.");
    let shift = ciphers::caesar::run_freq_analysis(&encrypted, &corpus);
    println!("{}", shift);
}

