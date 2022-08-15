use std::ops::Rem;

struct FizzBuzz<T> {
  div_a: T,
  div_b: T,
  zero: T,
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

impl ToFzStr<u32> for FizzBuzz<u32> {
  fn to_str(&self, x: u32) -> String {
    common_fz_str(x, self.div_a, self.div_b, self.zero)
  }
}

pub fn fizz_buzz(end :u32) {
  if end > 1 { fizz_buzz(end-1) }

  println!("{}", FizzBuzz { div_a: 3,div_b: 5, zero: 0 }.to_str(end))
}
