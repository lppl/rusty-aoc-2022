use std::collections::HashMap;

mod own_solutions;

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;

  test("first_solution", own_solutions::first_solution);
  test("second_solution", own_solutions::second_solution);
  test("third_solution", own_solutions::third_solution);

  Ok(())
}

fn test(name: &str, run: fn(&str) -> i32) {
  assert_eq!(1, run("aa"), "{}: `aa`", name);
  assert_eq!(3, run("aa\nbb"), "{}: `aa,bb`", name);
  assert_eq!(6, run("abac\ndefe"), "{}: `abac\ndefe`", name);
  assert_eq!(27, run("AA"), "{}: `AA`", name);
  assert_eq!(157, run(include_str!("input.txt")), "{}: input.txt", name)
}