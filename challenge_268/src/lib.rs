pub fn abbreviate(phrase: &str) -> String {
    phrase
    .replace('-', " ")
    .chars()
    .map(|c| if c.is_alphanumeric() || c.is_whitespace() || c == '\'' {c} else {' '})
    .collect::<String>()
    .split_whitespace()
    .flat_map(|word| {
     let is_all_caps = word.chars().all(|c| c.is_uppercase());
     word.chars()
    .enumerate()
    .filter_map( move |(i, c)|{
        if i == 0 ||(!is_all_caps && c.is_uppercase()) {
          Some(c.to_ascii_uppercase())
        } else{
            None
        }
    })
    })
    .collect()
}