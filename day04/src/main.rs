use std::ops::RangeInclusive;
use std::str::FromStr;

use color_eyre::eyre::eyre;

fn main() {
  println!("Found {} contained ranges.", count(include_str!("./input.txt")))
}

#[derive(Debug)]
struct Line {
  left: RangeInclusive<usize>,
  right: RangeInclusive<usize>,
}

impl Line {
  fn contains(&self) -> bool {
    (self.left.contains(self.right.start()) && self.left.contains(self.right.end())) ||
      (self.right.contains(self.left.start()) && self.right.contains(self.left.end()))
  }
}

impl FromStr for Line {
  type Err = color_eyre::Report;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let result = s
      .split(&[',', '-'][..])
      .map(|num| num.parse::<usize>())
      .filter_map(|opt| opt.ok())
      .collect::<Vec<_>>();

    if result.len() == 4 {
      Ok(Line {
        left: result[0]..=result[1],
        right: result[2]..=result[3],
      })
    } else {
      Err(eyre!("Line is incorrect"))
    }
  }
}

fn count(s: &str) -> usize {
  s.lines()
    .map(|str_line| Line::from_str(str_line))
    .filter_map(|line| line.ok())
    .filter(|line| line.contains())
    .count()
}

#[cfg(test)]
mod tests {
  use test_case::test_case;

  use crate::count;

  #[test_case("1-1,2-2")]
  #[test_case("1-4,5-9")]
  #[test_case("2-2,1-1")]
  #[test_case("8-9,6-7")]
  fn one_line_out_of_range(str: &str) {
    assert_eq!(0, count(str))
  }

  #[test_case("2-3,3-4")]
  #[test_case("1-5,2-9")]
  #[test_case("2-9,1-5")]
  fn one_line_partial_contain(str: &str) {
    assert_eq!(0, count(str))
  }

  #[test_case("2-2,2-2")]
  #[test_case("3-4,4-4")]
  #[test_case("8-9,7-9")]
  fn one_line_full_contain(str: &str) {
    assert_eq!(1, count(str))
  }

  #[test_case(0, "3-5,7-8\n2-2,3-3")]
  #[test_case(1, "3-5,7-8\n2-2,2-2")]
  #[test_case(2, "3-5,4-5\n2-2,2-2")]
  fn multi_line_full_contain(num: usize, str: &str) {
    assert_eq!(num, count(str))
  }

  #[test]
  fn test_advent_input() {
    assert_eq!(2, count(include_str!("input.txt")));
  }
}