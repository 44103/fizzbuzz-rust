pub fn fizz_buzz() {
  let res = (1..=100)
    .map(|x| -> String {
      match (x % 3, x % 5) {
        (0, 0) => format!("FizzBuzz"),
        (0, _) => format!("Fizz"),
        (_, 0) => format!("Buzz"),
        _ => format!("{x}"),
      }
    })
    .collect::<Vec<_>>()
    .join("\n");

  println!("{res}");
}
