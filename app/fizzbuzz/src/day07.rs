pub fn fizz_buzz() {
  for x in 1..=100 {
    let tmp;
    let s = match (x % 3, x % 5) {
      (0, 0) => "FizzBuzz",
      (0, _) => "Fizz",
      (_, 0) => "Buzz",
      _ => { tmp = x.to_string(); &tmp },
    };
    println!("{s}");
  }
}
