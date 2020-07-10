use std::collections::HashMap;

// Implements a simple caesar cipher. Takes an input message and shifts each letter
// by the specified amount. Only encodes a-zA-Z.
// TODO: Implement flag for including capital letters.
pub fn encipher(msg: &str, shift: u8) -> String {
    let out: String = msg
        .chars()
        .map(|c| {
            let case = if c.is_uppercase() { b'A' } else { b'a' };
            match c.is_ascii_alphabetic() {
                true => {
                    (case + ((c as u8 + shift - case)%26)) as char
                }
                false => {
                    c
                }
            }
        }).collect();
    return out;
    }

pub fn decipher(msg: &str, shift: u8) -> String {
    encipher(msg, 26 - shift)
}

pub fn kullback_leibler(dist1: Vec<f32>, dist2: Vec<f32>) -> f32 {
    dist1.iter().zip(dist2.iter()).map(|(p, q)| {
        match (p, q) {
            (&x, &y) if x==0.0_f32 || y==0.0_f32 => 0.0,
            _ => p * f32::log2(p/q)
        }
    }).sum()
}

pub fn run_freq_analysis(msg: &str, corpus: &str) -> u8 {
    let corpus_dist = &get_character_frequency(corpus);
    let mut lowest = f32::MAX;
    let mut best = 0;
    for i in 1..26 {
        let score = kullback_leibler(corpus_dist.to_vec(), get_character_frequency(&decipher(msg, i)));
        println!("{}", score);
        if score < lowest {
            lowest = score;
            best = i;
        }
    }
    best
}

//pub fn get_character_frequency(msg: &str) -> HashMap<char, f32> {
pub fn get_character_frequency(msg: &str) -> Vec<f32> {
    let mut counter = HashMap::new();
    let mut num_chars = 0.0;
    for c in msg.to_ascii_lowercase().chars() {
        match c.is_ascii_alphabetic() {
            true => { *counter.entry(c).or_insert(0.0) += 1.0;
                        num_chars += 1.0; }
            false => { continue }
        }
    }
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut freq_vec = Vec::new();
    // Pad freq map with 0 values for missing letters
    for letter in alphabet.chars() {
        if counter.contains_key(&letter) {
            freq_vec.push(counter[&letter]);
        } else {
            freq_vec.push(0.0);
        }
    }

    return freq_vec.into_iter().map(|x| { x / num_chars }).collect();
    // Divide counts by total number of characters to get frequencies.
    //for (_, count) in counter.iter_mut() {
      //  *count /= num_chars;
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encipher() {
        assert_eq!(encipher("abc", 1), "bcd");
        assert_eq!(encipher("a b c", 1), "b c d");
        assert_eq!(encipher("abc", 27), "bcd");
        assert_eq!(encipher("Alan Turing", 13), "Nyna Ghevat")
        }

    #[test]
    fn test_decipher() {
        assert_eq!(decipher("bcd", 1), "abc");
        assert_eq!(decipher("Nyna Ghevat", 13), "Alan Turing")
    }
}
