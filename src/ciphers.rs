use std::collections::HashMap;

//TODO: Historical ciphers would remove all capitals, spaces, and punctuation. The
//output would be formatted in 4 or 5 letter blocks.

// Implements a simple caesar cipher. Takes an input message and shifts each letter
// by the specified amount. Only encodes a-zA-Z.
// TODO: Implement flag for including capital letters.
pub mod caesar {
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

    pub fn run_freq_analysis(msg: &str, corpus: &str) -> u8 {
        let corpus_dist = &super::get_character_frequencies(corpus);
        let mut lowest = f32::MAX;
        let mut best = 0;
        for i in 1..26 {
            let score = super::kullback_leibler(corpus_dist.to_vec(), super::get_character_frequencies(&decipher(msg, i)));
            println!("{}", score);
            if score < lowest {
                lowest = score;
                best = i;
            }
        }
        best
    }
}

//TODO: Find better way than char.to_string()
//TODO: Fix error with caps
pub mod vigenere {
    pub fn encipher(msg: &str, key: &str) -> String {
        let mut key_iter = key.chars().cycle();
        let out: String = msg
            .chars()
            .map(|c| {
                let case = if c.is_uppercase() { b'A' } else { b'a' };
                match c.is_ascii_alphabetic() {
                    true => {
                        let shift = key_iter.next().unwrap() as u8 - case;
                        super::caesar::encipher(&c.to_string(), shift)
                    }
                    false => {
                        c.to_string()
                    }}}).collect();
        return out;
    }

    // Shifts each char in the key to the corresponding
    // value to reverse.
    pub fn decipher(msg: &str, key: &str) -> String {
        let d_key: String = key.chars().map(|c| {
            // This likely allocates case for each char?
            let case = if c.is_uppercase() { b'A' } else { b'a' };
            (26 - (c as u8 - case) + case) as char
        }).collect();
        encipher(msg, &d_key)
    }
}

// Calculate the kl divergence for two distributions. Lower means
// the distributions are more similar.
fn kullback_leibler(dist1: Vec<f32>, dist2: Vec<f32>) -> f32 {
    dist1.iter().zip(dist2.iter()).map(|(p, q)| {
        match (p, q) {
            (&x, &y) if x==0.0_f32 || y==0.0_f32 => 0.0,
            _ => p * f32::log2(p/q)
        }
    }).sum()
}

// Handling caps would better reflect the underlying structure of language.
fn get_character_frequencies(msg: &str) -> Vec<f32> {
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
    // Pad freq dist vec with 0 values for missing letters
    for letter in alphabet.chars() {
        if counter.contains_key(&letter) {
            freq_vec.push(counter[&letter]);
        } else {
            freq_vec.push(0.0);
        }
    }
    return freq_vec.into_iter().map(|x| { x / num_chars }).collect();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_caesar_encipher() {
        assert_eq!(super::caesar::encipher("abc", 1), "bcd");
        assert_eq!(super::caesar::encipher("a b c", 1), "b c d");
        assert_eq!(super::caesar::encipher("abc", 27), "bcd");
        assert_eq!(super::caesar::encipher("Alan Turing", 13), "Nyna Ghevat");
        }

    #[test]
    fn test_caesar_decipher() {
        assert_eq!(super::caesar::decipher("bcd", 1), "abc");
        assert_eq!(super::caesar::decipher("Nyna Ghevat", 13), "Alan Turing")
    }

    #[test]
    fn test_vigenere() {
        assert_eq!(super::vigenere::encipher("attackatdawn", "lemon"), "lxfopvefrnhr");
//        assert_eq!(super::vigenere::encipher("Attackatdawn", "lemon"), "Lxfopvefrnhr");
        assert_eq!(super::vigenere::decipher("lxfopvefrnhr", "lemon"), "attackatdawn")
    }
}
