use std::fs;
mod caesar;

fn main() {
    let encrypted = caesar::encipher("In the classical approach to geometry, the measure of a body was often computed by partitioning thatbody into finitely many components, moving around each componentby a rigid motion (e.g.  a translation or rotation), and then reassembling  those  components  to  form  a  simpler  body  which  presumablyhas the same area also ", 87);
    println!("{}", encrypted);
    let corpus = fs::read_to_string("eng_text_sample.txt").expect("Error reading file.");
    let shift = caesar::run_freq_analysis(&encrypted, &corpus);
    println!("{}", shift);
}

