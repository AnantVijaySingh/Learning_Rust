// Given a list of strings find which ones are anagrams of each other
// This is a revision exercise so the goal is to make it as complicated as possible to try
// and use everything I have learnt till this point :D

use std::collections::HashMap;

fn main() {

    let words: Vec<&str> = vec![
        "apple",
        "banana",
        "cherry",
        "dog",
        "elephant",
        "grape",
        "hamburger",
        "kiwi",
        "lemon",
        "melon",   // Anagram of "lemon"
        "listen",  // Anagram of "silent"
        "silent",  // Anagram of "listen"
        "debit card", // Anagram of "badcredit"
        "bad credit", // Anagram of "debitcard"
    ];


    println!("{:?}", words);

    for count in 0..words.len()-1 {
        for count_plus_one in count+1..words.len() {
            is_an_anagram_or_not(words[count], words[count_plus_one])
        }
    }
}

fn is_an_anagram_or_not(word_1: &str, word_2: &str) {

    let mut alphabet_list = HashMap::from([
        ('a', [0, 0]),
        ('b', [0, 0]),
        ('c', [0, 0]),
        ('d', [0, 0]),
        ('e', [0, 0]),
        ('f', [0, 0]),
        ('g', [0, 0]),
        ('h', [0, 0]),
        ('i', [0, 0]),
        ('j', [0, 0]),
        ('k', [0, 0]),
        ('l', [0, 0]),
        ('m', [0, 0]),
        ('n', [0, 0]),
        ('o', [0, 0]),
        ('p', [0, 0]),
        ('q', [0, 0]),
        ('r', [0, 0]),
        ('s', [0, 0]),
        ('t', [0, 0]),
        ('u', [0, 0]),
        ('v', [0, 0]),
        ('w', [0, 0]),
        ('x', [0, 0]),
        ('y', [0, 0]),
        ('z', [0, 0]),
        (' ', [0, 0])
    ]);

    for char in word_1.chars() {
        alphabet_list.entry(char).and_modify(|value| {
            value[0] += 1;
        });
    }

    for char in word_2.chars() {
        alphabet_list.entry(char).and_modify(|value| {
            value[1] += 1;
        });
    }

    for (_key, value) in &alphabet_list {
        if value[0] != value[1] {
            // println!("{} and {} are not anagrams", word_1, word_2);
            return;
        }
    }

    println!("-----> {} and {} are anagrams", word_1, word_2);
}
