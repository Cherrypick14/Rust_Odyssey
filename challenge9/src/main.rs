fn main() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = (&t.0, &t.1); // borrowing instead of moving

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
