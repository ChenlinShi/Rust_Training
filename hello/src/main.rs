use std::io::Write; // write trait
use std::str::FromStr; // fromstr trait
fn main() {
 let mut numbers = Vec::new(); // vector
 for arg in std::env::args().skip(1) { // iterate the arguments, skip the first one(args[0], the name of the program)
 numbers.push(u64::from_str(&arg) // push the argument to the vector, make the argument to u64
 .expect("error parsing argument")); // if error, print the message
 }
 if numbers.len() == 0 {
 writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();// unwrap() is used to examine if writeln!() is successful
 std::process::exit(1);
 }

 let mut d = numbers[0];
 for m in &numbers[1..] { // & is used to borrow the value, not move the value
 d = gcd(d, *m); // * is used to dereference the value
 }
 println!("The greatest common divisor of {:?} is {}",
 numbers, d);
}
fn gcd (mut n:u64, mut m: u64)-> u64{
    assert! (n!=0 && m!=0);
    while m!=0 {
        if m<n {
          let t=m;
          m=n;
          n=t;
        }

        m=m%n;
    } 
    n // return value, no semicolon
}
// return is used when you need to return early from a function

#[test] // test attribute
fn test_gcd() {
 assert_eq!(gcd(14, 15), 1);
 assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
 3 * 7 * 11 * 13 * 19),
 3 * 11);
}

