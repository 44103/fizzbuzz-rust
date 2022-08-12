pub fn fizz_buzz() {
  use std::ops::Rem;

  fn fz<T>(x: T, div_a: T, div_b: T, zero: T) -> String
  where T: Rem<T, Output=T> + Eq + Copy + ToString {
    match (x % div_a == zero, x % div_b == zero) {
      (true, true) => format!("FizzBuzz"),
      (true, _) => format!("Fizz"),
      (_, true) => format!("Buzz"),
      _ => x.to_string(),
    }
  }

  (1..=100)
  .map(|x: u32| fz(x, 3, 5 ,0))
  .for_each(|x| println!("{x}"))
}
