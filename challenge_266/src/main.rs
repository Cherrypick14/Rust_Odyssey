
// fn counting_words(words: &str) -> HashMap<String, u32> {
//     // create an ew hashmap
//     //split into whitesapce
//     //convert to lowercase
//     //dereference the word_count

//     let mut word_count = HashMap::new();

//     for word in words.split_whitespace() {
//         let word = words.to_lowercase();
//         *word_count.entry(words.to_string()).or_insert(0) += 1;
//     }
//     word_count
// }

fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for word in words
        .to_lowercase()
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\''))
    {
        if !word.is_empty() {
            *map.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    map
}

   
  
// use counting_words::counting_words;
use std::collections::HashMap;

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.”
    ― Albert Einstein "));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}


// the output should be this one:

// $ cargo run
// {"hello": 1, "world": 1}
// {"and": 2, "human": 1, "universe": 2, "the": 2, "i\'m": 1, "about": 1, "einstein": 1, "are": 1, "infinite": 1, "sure": 1, "albert": 1, "two": 1, "things": 1, "not": 1, "stupidity": 1}
// {"batman": 3, "stop": 2}
// $