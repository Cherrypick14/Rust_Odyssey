pub fn square_of_sum(n: u32) -> u32 {
    // use the formula n*(n+1) / 2
    let sum = n * (n + 1) / 2 ;
    sum*sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    // Use the formula n * (n + 1) (2n + 1)
    n * (n + 1) * (2 * n + 1) / 6 
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
