use std::collections::HashMap;

pub fn first_solution(s: &str) -> i32 {
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

pub fn second_solution(s: &str) -> i32 {
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