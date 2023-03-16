use std::collections::HashMap;

fn first_solution(s: &str) -> i32 {
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

fn second_solution(s: &str) -> i32 {
  let mut i = 0;
  let letters: HashMap<char, i32> = HashMap::from_iter(
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map(|letter| {
      i += 1;
      (letter, i)
    })
  );
  s.lines()
    .map(|line| line.split_at(line.len() / 2))
    .map(|(left, right)| left.chars().find(|lc| right.chars().any(|rc| lc == &rc)))
    .map(|letter| {
      match letter {
        Some(letter) => letters.get(&letter),
        None => Some(&0),
      }
    })
    .map(|num| {
      match num {
        Some(num) => num,
        None => &0,
      }
    })
    .sum()
}

fn third_solution(s: &str) -> i32 {
  let mut i = 0;
  let letters: HashMap<char, i32> = HashMap::from_iter(
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map(|letter| {
      i += 1;
      (letter, i)
    })
  );
  s.lines()
    .map(|line| line.split_at(line.len() / 2))
    .flat_map(|(left, right)| left.chars().find(|lc| right.chars().any(|rc| lc == &rc)))
    .flat_map(|letter| letters.get(&letter))
    .sum()
}

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;

  test("first_solution", first_solution);
  test("second_solution", second_solution);
  test("third_solution", third_solution);

  Ok(())
}

fn test(name: &str, run: fn(&str) -> i32) {
  assert_eq!(1, run("aa"), "{}: `aa`", name);
  assert_eq!(3, run("aa\nbb"), "{}: `aa,bb`", name);
  assert_eq!(6, run("abac\ndefe"), "{}: `abac\ndefe`", name);
  assert_eq!(27, run("AA"), "{}: `AA`", name);
  assert_eq!(157, run(include_str!("input.txt")), "{}: input.txt", name)
}