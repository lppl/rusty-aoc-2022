use std::collections::HashMap;

fn run(s: &str) -> i32 {
  let mut i = 0;
  let letters: HashMap<char, i32> = HashMap::from_iter(
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map(|letter| {
      i += 1;
      (letter, i)
    })
  );
  s.lines()
    .map(|line| {
      let (left, right) = line.split_at(line.len() / 2);
      match left.chars().find(|lc| right.chars().any(|rc| lc == &rc)) {
        Some(key) => {
          match letters.get(&key) {
            Some(num) => num,
            None => &0,
          }
        }
        None => &0,
      }
    })
    .sum()
}

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;

  test("first_solution", run);

  Ok(())
}

fn test(name: &str, run: fn(&str) -> i32) {
  assert_eq!(1, run("aa"), "{}: `aa`", name);
  assert_eq!(3, run("aa\nbb"), "{}: `aa,bb`", name);
  assert_eq!(6, run("abac\ndefe"), "{}: `abac\ndefe`", name);
  assert_eq!(27, run("AA"), "{}: `AA`", name);
  assert_eq!(157, run(include_str!("input.txt")), "{}: input.txt", name)
}