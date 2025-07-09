use std::collections::HashMap;

fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for word in words
        .to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
    {
        if !word.is_empty() {
            *map.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    map
}

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.” ― Albert Einstein"));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}
