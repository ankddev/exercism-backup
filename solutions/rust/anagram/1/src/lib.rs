use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .into_iter()
        .filter(|el| is_anagram(word, el))
        .map(|el| *el)
        .collect()
}

fn is_anagram(one: &str, other: &str) -> bool {
    one.to_lowercase() != other.to_lowercase() && sorted_chars(one) == sorted_chars(other)
}

fn sorted_chars(word: &str) -> String {
    let mut chars: Vec<_> = word.to_lowercase().chars().collect();
    chars.sort();
    chars.into_iter().collect()
}
