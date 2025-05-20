use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize(word);
    let lower_word = word.to_lowercase();

    possible_anagrams
        .iter()
        .copied()
        .filter(|&candidate| {
            let lower_candidate = candidate.to_lowercase();

            // Exclude if the lowercase versions are the same
            if lower_candidate == lower_word {
                return false;
            }

            let normalized_candidate = normalize(candidate);
            normalized_candidate == normalized_word
        })
        .collect()
}

fn normalize(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}
