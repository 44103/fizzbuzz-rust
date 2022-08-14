use std::ops::Rem;

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

fn common_fz_str<T>(x: T, div_a: T, div_b: T, zero: T) -> String
where T: Rem<T, Output=T> + Eq + Copy + ToString {
  match (x % div_a == zero, x % div_b == zero) {
    (true, true) => format!("FizzBuzz"),
    (true, _) => format!("Fizz"),
    (_, true) => format!("Buzz"),
    _ => x.to_string(),
  }
}

trait ToFzStr<T> {
  fn to_str(&self, x: T) -> String;
}

impl ToFzStr<i32> for FizzBuzz<i32> {
  fn to_str(&self, x: i32) -> String {
    common_fz_str(x, self.div_a, self.div_b, self.zero)
  }
}

pub fn fizz_buzz(end: usize) {
  (1..)
  .take(end)
  .map(|x| FizzBuzz::new(3, 5, 0).to_str(x))
  .for_each(|x| println!("{x}"))
}
