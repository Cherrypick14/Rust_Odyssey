fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Modify it here:
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}