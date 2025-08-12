pub fn abbreviate(phrase: &str) -> String {
    phrase
        // Replace hyphens with spaces so they're treated as separators
        .replace('-', " ")
        // Remove punctuation except spaces and alphanumeric characters
        .chars()
        .map(|c| if c.is_alphabetic() || c.is_whitespace() { c } else { ' ' })
        .collect::<String>()
        // Split into words
        .split_whitespace()
        // Take the first letter of each word and uppercase it
        .map(|word| word.chars().next().unwrap().to_ascii_uppercase())
        .collect()
}
