pub struct Matrix(pub Vec<Vec<i32>>); // defining a tuple struct

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let data = slice.iter().map(|row| row.to_vec()).collect();

        Matrix(data)
    }
}

use std::fmt::{Display, Formatter, Result};

impl Display for Matrix {
     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in &self.0 {
            writeln!(f, "({})", row.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "))?;
        }
        Ok(())
    }
}

fn main() {
    let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
    println!("{}", matrix);
}