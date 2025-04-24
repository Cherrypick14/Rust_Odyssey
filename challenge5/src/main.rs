fn main() {
    let x = (1, 2, (), "hello"); // use a string slice (&str) instead of String
    let y = x; // no clone needed since x is copied
    println!("{:?}, {:?}", x, y);
}
