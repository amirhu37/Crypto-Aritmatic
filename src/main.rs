use itertools::Itertools;
use std::collections::HashSet;

// Helper function to convert a word to its numerical value based on current assignment
fn word_to_number(word: &str, mapping: &[(char, u32)]) -> u32 {
    let mut number: u32 = 0;
    for c in word.chars() {
        let digit: u32 = mapping.iter().find(|&&(ch, _)| ch == c).unwrap().1;
        number = number * 10 + digit;
    }
    number
}

// Function to check if current mapping satisfies the puzzle
fn is_valid_solution(words: &[&str], result: &str, mapping: &[(char, u32)]) -> bool {
    let sum: u32 = words.iter().map(|&word| word_to_number(word, mapping)).sum();
    sum == word_to_number(result, mapping)
}

// Function to generate permutations of digits and find a valid solution
fn solve_crypto_arithmetic(words: &[&str], result: &str) -> Option<Vec<(char, u32)>> {
    let mut letters: HashSet<char> = HashSet::new();
    for &word in words.iter().chain(std::iter::once(&result)) {
        for c in word.chars() {
            letters.insert(c);
        }
    }

    let letters: Vec<char> = letters.into_iter().collect();
    if letters.len() > 10 {
        return None; // More than 10 unique letters, no solution possible with single digits
    }

    let digits: Vec<u32> = (0..10).collect();
    let permutations: itertools::Permutations<std::iter::Cloned<std::slice::Iter<u32>>> = digits.iter().cloned().permutations(letters.len());

    for perm in permutations {
        let mapping: Vec<(char, u32)> = letters.iter().cloned().zip(perm).collect();
        if is_valid_solution(words, result, &mapping) {
            return Some(mapping);
        }
    }
    None
}

fn main() {
    let words: Vec<&str> = vec!["TWO", "TWO"];
    let result: &str = "FOUR";
    println!("{} + {} = {}", words[0], words[1], result);
    
    match solve_crypto_arithmetic(&words, result) {
        Some(mapping) => {
            // Extract characters and form a string
            let extracted_string: String = mapping.iter().map(|&(ch, _)| ch).collect();
            println!("Solution found: {} " , extracted_string);
            for (ch, digit) in mapping {
                println!("{} = {}", ch, digit);
            }
        }
        None => println!("No solution found."),
    }
}



