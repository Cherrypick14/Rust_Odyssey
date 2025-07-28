
// Define a struct named Matrix as a tuple of 2 tuples. The nested tuple will contain 2 i32s.
#[derive(Debug)]
struct Matrix((i32, i32), (i32, i32)) ; 


 fn transpose(m: Matrix) -> Matrix {
     Matrix(
        (m.0 .0, m.1 .0),
        (m.0 .1, m.1 .1)
     )
}

fn main() {
    let matrix = Matrix((1, 3), (4, 5));
    println!("Original matrix {:?}", matrix);
    println!("Transpose matrix {:?}", transpose(matrix));
}