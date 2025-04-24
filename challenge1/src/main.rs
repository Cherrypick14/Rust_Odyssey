fn main() {
    // Create a String
    let x = String::from("Hello world");
    
    // Clone x into y, so both x and y have their own copy
    let y = x.clone();
    
    // Now we can safely print both
    println!("{}, {}", x, y);
}
