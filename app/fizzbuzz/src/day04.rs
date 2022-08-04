pub fn fizz_buzz() {
  for x in 1..=100 {
    match x {
      e if e % 15 == 0 => println!("FizzBuzz"),
      e if e % 3 == 0 => println!("Fizz"),
      e if e % 5 == 0 => println!("Buzz"),
      e => println!("{e}"),
    }
  }
}
