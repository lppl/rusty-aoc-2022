mod first_solution;
mod second_solution;

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;

  test("first_solution", first_solution::rate_game);
  test("second_solution", second_solution::rate_game);

  Ok(())
}

fn test(name: &str, rate_game: fn(&str) -> i32) {
  assert_eq!(4, rate_game("A X"), "{}: rate draw using rocks", name);
  assert_eq!(5, rate_game("B Y"), "{}: rate draw using papers", name);
  assert_eq!(6, rate_game("C Z"), "{}: rate draw using scissors", name);

  assert_eq!(7, rate_game("C X"), "{}: rate winning using rocks", name);
  assert_eq!(8, rate_game("A Y"), "{}: rate winning using papers", name);
  assert_eq!(9, rate_game("B Z"), "{}: rate winning using scissors", name);

  assert_eq!(1, rate_game("B X"), "{}: rate losing using rocks", name);
  assert_eq!(2, rate_game("C Y"), "{}: rate losing using papers", name);
  assert_eq!(3, rate_game("A Z"), "{}: rate losing using scissors", name);

  assert_eq!(15, rate_game("A Y\nB X\nC Z"), "{}: rate game for 15 points", name);
}