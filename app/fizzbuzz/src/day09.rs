pub fn fizz_buzz() {
  let res = (1..=100)
    .fold(format!(""), |buf, x| {
      match (x % 3, x % 5) {
        (0, 0) => format!("{buf}FizzBuzz\n"),
        (0, _) => format!("{buf}Fizz\n"),
        (_, 0) => format!("{buf}Buzz\n"),
        _ => format!("{buf}{x}\n"),
      }
    });

    println!("{res}");
}
