use std::fmt::Formatter;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
struct Item(u8);

impl TryFrom<u8> for Item {
  type Error = color_eyre::Report;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      b'a'..=b'z' => Ok(Item(value)),
      b'A'..=b'Z' => Ok(Item(value)),
      _ => Err(color_eyre::eyre::eyre!("{} is not a valid item", value as char)),
    }
  }
}

impl Item {
  fn value(self) -> usize {
    match self {
      Item(b'a'..=b'z') => 1 + ((self.0 - b'a') as usize),
      Item(b'A'..=b'Z') => 27 + ((self.0 - b'A') as usize),
      _ => unreachable!(),
    }
  }
}

impl std::fmt::Debug for Item {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

pub fn solve(s: &str) -> i32 {
  s.lines()
    .map(|line| {
      let x = line
        .bytes()
        .map(|c| Item::try_from(c))
        .filter_map(|item| item.ok())
        .collect_vec();

      let (left, right) = x.split_at(x.len() / 2);

      let x = left
        .iter()
        .filter(|item| right.contains(item))
        .next();

      match x {
        Some(item) => item.value() as i32,
        None => unreachable!(),
      }
    })
    .sum::<i32>()
}