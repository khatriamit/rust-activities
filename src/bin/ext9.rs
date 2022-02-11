// check the words entered by user for anagram in RUST

/*
    Anagram are the words that spells exactly the same when shuffled.
    For example listen and silent have same letters if compared precisely
*/

use std::iter::FromIterator;
use std::iter::Iterator;

fn main() {
    let mut first_word = String::new();
    println!("Enter first word:");
    std::io::stdin().read_line(&mut first_word).unwrap();
    let mut second_word = String::new();
    println!("Enter second word:");
    std::io::stdin().read_line(&mut second_word).unwrap();

    let first_slice: &str = &first_word[..];
    let second_slice: &str = &second_word[..];

    let mut first_chars: Vec<char> = first_slice.chars().collect();
    let mut second_chars: Vec<char> = second_slice.chars().collect();
    first_chars.sort_by(|a, b| b.cmp(a));
    second_chars.sort_by(|a, b| b.cmp(a));

    let first = String::from_iter(first_chars);
    let second = String::from_iter(second_chars);

    if first == second {
        println!("the words are anagrams");
    } else {
        println!("the words are not anagrams")
    }
}
