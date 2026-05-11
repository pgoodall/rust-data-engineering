use clap::Parser;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// A list of numbers with some numbers repeated
    #[arg(short, long)]
    pub num_list: Option<String>,

    /// Count occurances of a word in a short sentence
    #[arg(short, long)]
    pub sentence: Option<String>,

    /// Word for which to search in a supplied sentence or file
    #[arg(short, long)]
    pub word: Option<String>,

    /// Search the given file for a given word
    #[arg(short, long)]
    pub file: Option<String>,
}

pub fn frequency<T>(content: Vec<T>) -> HashMap<T, i32> 
where
    T: Eq + PartialEq + Hash,
{
    let mut frequencies = HashMap::new();
    for k in content {
            let count = frequencies.entry(k).or_insert(0);
            *count += 1;
        }

    frequencies
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_frequency() {
        let numbers = vec![1, 2, 3, 4, 1, 2, 3, 1, 2, 1];
        let map = frequency(numbers);
        let result = map.get(&1).unwrap().clone();
        assert_eq!(4, result)
    }

    #[test]
    fn test_word_frequency() {
        let sentence = "hello world wonderful world";
        let map = frequency(sentence.split_whitespace().collect());
        let result = map.get("world").unwrap().clone();
        assert_eq!(2, result)
    }
}
