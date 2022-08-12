struct FizzBuzz<T> {
  div_a: T,
  div_b: T,
  zero: T,
}

impl<T> FizzBuzz<T> {
  fn new(div_a: T, div_b: T, zero: T) -> Self {
    FizzBuzz { div_a, div_b, zero }
  }
}

trait ToFzStr<T> {
  fn to_str(&self, x: T) -> String;
}

impl ToFzStr<i32> for FizzBuzz<i32> {
  fn to_str(&self, x: i32) -> String {
    match (x % self.div_a == self.zero, x % self.div_b == self.zero) {
      (true, true) => format!("FizzBuzz"),
      (true, _) => format!("Fizz"),
      (_, true) => format!("Buzz"),
      _ => x.to_string(),
    }
  }
}

pub fn fizz_buzz() {
  (1..=100)
  .map(|x| FizzBuzz::new(3, 5, 0).to_str(x))
  .for_each(|x| println!("{x}"))
}
