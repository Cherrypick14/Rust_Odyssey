use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    if h.is_empty() {
        return i32::MAX;
    }

    let mut min = i32::MAX;
    for value in h.values() {
        if *value < min {
            min = *value;
        }
    }

    min
}

fn main() {
    let mut hash = HashMap::new();
    hash.insert("Cat", 122);
    hash.insert("Dog", 333);
    hash.insert("Elephant", 334);
    hash.insert("Gorilla", 14);

    println!("The smallest of the elements in the HashMap is {}", smallest(hash));
}
