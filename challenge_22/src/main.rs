fn prev_prime(mut nbr: u64) -> u64 {
   while !is_prime(nbr) {
         nbr -= 1;
   }

   nbr
}

 fn is_prime(nbr: u64) -> bool {
     if nbr < 2 {
        return false;
     }

     let sqrt: u64 = (nbr as u64).isqrt() as u64;

     for i in 2..=sqrt {
        if nbr % i == 0 {
            return false;
        }
     }
      true
 }

   fn main() {
    println!("The previous prime number before 34 is: {}", prev_prime(34));

}