pub fn fizz_buzz() {
  for x in 1..=100 {
    let s = match (x % 3, x % 5) {
      (0, 0) => "FizzBuzz".to_string(),
      (0, _) => "Fizz".to_string(),
      (_, 0) => "Buzz".to_string(),
      _ => x.to_string(),
    };
    println!("{s}")
  }
}
