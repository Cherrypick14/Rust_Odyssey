
fn next_prime(mut nbr: u64) -> u64 {
  
 // Start from nbr and go up until we find a prime
   while !is_prime(nbr) {
        nbr += 1;
   }  

   nbr
  
}

fn is_prime(nbr: u64) -> bool {
    if nbr < 2 {
        return false;
    }

    let sqrt = (nbr as u64).isqrt() as u64 ; 

    for i in 2..=sqrt {
        if nbr % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("The next prime after 4 is: {}", next_prime(4));
    println!("The next prime after 11 is: {}", next_prime(11));
}