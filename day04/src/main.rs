use std::ops::RangeInclusive;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
  println!("Found {} contained ranges.", count(include_str!("./input.txt")))
}

pub trait RangeInclusiveExt {
  fn contains_range(&self, other: &Self) -> bool;

  fn contains_or_is_contained(&self, other: &Self) -> bool {
    self.contains_range(&other) || other.contains_range(&self)
  }
}

impl<T> RangeInclusiveExt for RangeInclusive<T> where T: PartialOrd {
  fn contains_range(&self, other: &Self) -> bool {
    self.contains(other.start()) && self.contains(other.end())
  }
}

#[derive(Debug)]
struct Line {
  left: RangeInclusive<usize>,
  right: RangeInclusive<usize>,
}

impl Line {
  fn have_contained_ranges(&self) -> bool {
    self.left.contains_or_is_contained(&self.right)
  }
}

impl FromStr for Line {
  type Err = color_eyre::Report;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (left, right) = s
      .split(',')
      .map(|s| {
        let (start, end) = s
          .split('-')
          .map(|s| s.parse().expect("range start/end should be u32"))
          .collect_tuple::<(usize, usize)>()
          .expect("something");
        start..=end
      })
      .collect_tuple::<(RangeInclusive<usize>, RangeInclusive<usize>)>()
      .expect("Incorrect line format");

    Ok(Line { left, right })
  }
}

fn count(s: &str) -> usize {
  s.lines()
    .map(|str_line| Line::from_str(str_line))
    .filter_ok(|line| line.have_contained_ranges())
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