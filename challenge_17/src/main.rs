
fn reverse_it(v: i32) -> String {
   let abs_string: String = v.abs().to_String();
   let reversed_string: String = abs_string.chars().rev().collect();

   if v < 0 {
     format!("-{}{}". reversed_string, abs_string)
   } else {
     format!("{}{}", reversed_string, abs_string)
   }
}

fn main(){
   println!("{}", reverse_it(123));
   println!("{}", reverse_it(-123));
}