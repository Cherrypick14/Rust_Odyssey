fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial == 0 || factorial == 1 {
        return 0;
    } 
   
    let mut result = 1 ;
    let mut i = 1 ;

    while factorial < result {
        i += 1;
        result *= i ;

        if result == factorial {
            return i ; 
        }
    }
      0
}


fn main() {
    println!("The factorial steps of 720 = {}", count_factorial_steps(720)); // 6
    println!("The factorial steps of 13 = {}", count_factorial_steps(13));   // 0
    println!("The factorial steps of 6 = {}", count_factorial_steps(6));     // 3
}
