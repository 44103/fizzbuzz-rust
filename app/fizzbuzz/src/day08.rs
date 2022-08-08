pub fn fizz_buzz() {
  (1..=100).map(|x: i32| {
    match (x % 3, x % 5) {
      (0, 0) => format!("FizzBuzz"),
      (0, _) => format!("Fizz"),
      (_, 0) => format!("Buzz"),
      _ => format!("{x}"),
    }
  }).for_each(|x| println!("{x}"));
}
