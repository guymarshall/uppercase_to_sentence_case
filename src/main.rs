use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn to_sentence_case(word: String) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().chain(chars.as_str().to_lowercase().chars()).collect(),
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let mut names = BufReader::new(input_file)
        .lines()
        .map(Result::unwrap)
        .map(to_sentence_case)
        .collect::<Vec<String>>();

    names.sort();

    let mut output_file = File::create("output.txt").unwrap();
    for name in names {
        writeln!(output_file, "{}", name).unwrap();
    }
}
