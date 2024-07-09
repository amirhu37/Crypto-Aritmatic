use itertools::Itertools;
use std::process::Command;
use std::collections::HashSet;

/// A macro to prompt for user input with an optional message.
///
/// # Examples
///
/// ```
/// let name: String = input!("Enter your name: ");
/// println!("Hello, {}!", name);
/// ```
#[macro_export]
macro_rules! input {
    ($input_type:ty) => {
        input!($input_type, "");
    };
    () => {
        input!(String, "");
    };
    ($msg:expr) => {
        input!(String, $msg)
    };
    ($input_type:ty, $msg:expr) => {{
        use std::io::{self, Write};
        print!("{}", $msg);
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");
        let trimmed_input = user_input.trim().to_string();
        match trimmed_input.parse::<$input_type>() {
            Ok(r) => r,
            Err(error) => panic!("Cannot parse: {:?}", error),
        }
    }};
}

/// Converts a word to its numerical value based on the given character-to-digit mapping.
///
/// # Parameters
///
/// - `word`: A string slice representing the word to convert.
/// - `mapping`: A slice of tuples, each containing a character and its corresponding digit.
///
/// # Returns
///
/// The numerical value of the word as a `u32`.
///
/// # Examples
///
/// ```
/// let mapping = [('A', 1), ('B', 2), ('C', 3)];
/// let result = word_to_number("ABC", &mapping);
/// assert_eq!(result, 123);
/// ```
fn word_to_number(word: &str, mapping: &[(char, u32)]) -> u32 {
    let mut number: u32 = 0;
    for c in word.chars() {
        let digit: u32 = mapping.iter().find(|&&(ch, _)| ch == c).unwrap().1;
        number = number * 10 + digit;
    }
    number
}

/// Checks if the current mapping satisfies the puzzle.
///
/// # Parameters
///
/// - `words`: A vector of strings representing the words.
/// - `result`: A string representing the result word.
/// - `mapping`: A slice of tuples, each containing a character and its corresponding digit.
///
/// # Returns
///
/// `true` if the solution is valid, otherwise `false`.
///
/// # Examples
///
/// ```
/// let words = vec!["SEND".to_string(), "MORE".to_string()];
/// let result = "MONEY".to_string();
/// let mapping = [('S', 9), ('E', 5), ('N', 6), ('D', 7), ('M', 1), ('O', 0), ('R', 8), ('Y', 2)];
/// assert!(is_valid_solution(&words, &result, &mapping));
/// ```
fn is_valid_solution(words: &Vec<String>, result: &String, mapping: &[(char, u32)]) -> bool {
    let words_sum: u32 = words.iter().map(|word| word_to_number(word, mapping)).sum();
    let result_value: u32 = word_to_number(result, mapping);
    words_sum == result_value
}

/// Solves the crypto-arithmetic puzzle for the given words and result.
///
/// # Parameters
///
/// - `words`: A vector of strings representing the words.
/// - `result`: A string representing the result word.
///
/// # Returns
///
/// An optional vector of tuples, each containing a character and its corresponding digit.
///
/// # Examples
///
/// ```
/// let words = vec!["SEND".to_string(), "MORE".to_string()];
/// let result = "MONEY".to_string();
/// if let Some(solution) = solve_crypto_arithmetic(words, result) {
///     for (ch, digit) in solution {
///         println!("{} = {}", ch, digit);
///     }
/// } else {
///     println!("No solution found.");
/// }
/// ```
fn solve_crypto_arithmetic(words: Vec<String>, result: String) -> Option<Vec<(char, u32)>> {
    let mut letters: HashSet<char> = HashSet::new();
    for word in &words {
        for c in word.chars() {
            letters.insert(c);
        }
    }
    for c in result.chars() {
        letters.insert(c);
    }
    let letters: Vec<char> = letters.into_iter().collect();
    if letters.len() > 10 {
        return None;
    }

    let digits: Vec<u32> = (0..10).collect();
    let permutations: itertools::Permutations<std::iter::Cloned<std::slice::Iter<u32>>> = digits.iter().cloned().permutations(letters.len());

    for perm in permutations {
        let mapping: Vec<(char, u32)> = letters.iter().cloned().zip(perm).collect();
        if is_valid_solution(&words, &result, &mapping) {
            return Some(mapping);
        }
    }
    None
}

/// Prompts the user for input and returns the words and result as a tuple.
///
/// # Returns
///
/// A tuple containing a vector of two words and the result word.
///
/// # Examples
///
/// ```
/// let (words, result) = inputs();
/// println!("Words: {:?}, Result: {}", words, result);
/// ```
fn inputs() -> (Vec<String>, String) {
    let input1: String = input!("Two Words as Input, Separated with Whitespace? ");
    let words: Vec<String> = input1.split_whitespace().map(String::from).collect();
    let result: String = input!("Result String? ");
    (words, result)
}

/// Clears the terminal screen. Only works on Windows.
fn cls() {
    Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .unwrap();
}

/// The main function to execute the program.
fn main() {
    cls();
    let (words, result) = inputs();
    // let (words, result) = (vec!["Send".to_string() , "more".to_string()] , "monry".to_string());

    println!("{} + {} = {}", words[0], words[1], result);
    match solve_crypto_arithmetic(words, result) {
        Some(mapping) => {
            let extracted_string: String = mapping.iter().map(|&(ch, _)| ch).collect();
            println!("Solution found: {}", extracted_string);
            for (ch, digit) in mapping {
                println!("{} = {}", ch, digit);
            }
        }
        None => println!("No solution found."),
    }
}
