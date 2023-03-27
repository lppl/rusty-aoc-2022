use itertools::{ cloned, Itertools };

// struct Crate {}
//
// struct Stack {}
//
// struct Command {
//   count: usize,
//   from: usize,
//   to: usize,
// }
//
// enum Cell {
//   Crate,
//   StackName,
//   Command,
//   Delimiter,
// }

fn run(input: &str) -> Vec<Option<char>> {
  let lines = input.lines();

  let y = lines
    .map(|line| {
      line
        .bytes()
        .chunks(4)
        .into_iter()
        .map(|chunk| {
          let mut iter = chunk.into_iter();
          let fuck = (iter.next(), iter.next(), iter.next());
          let (Some(l), Some(c), r) = fuck else {
            panic!("Hmmm destructuriong failed {:?}", fuck);
          };
          match (l, c, r) {
            (b' ', b' ', Some(b' ')) => None,
            (b' ', b' ', None) => None,
            (b' ', b'1'..=b'9', Some(b' ')) => Some(c as char),
            (b' ', b'1'..=b'9', None) => Some(c as char),
            (b'[', b'A'..=b'Z', Some(b']')) => Some(c as char),
            (b'[', b'A'..=b'Z', None) => Some(c as char),
            _ =>
              unreachable!(
                "Unreachable for '{l:?}' '{c:?}' '{r:?}', {:?}",
                vec![l, c].into_iter().map_into::<char>().join("")
              ),
          }
        })
        .collect::<Vec<Option<char>>>()
    })
    .collect::<Vec<Vec<Option<char>>>>();
  match y.first() {
    Some(x) => x.to_vec(),
    None => unreachable!("There should be always at least one line"),
  }
}

/*
============================================================================
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
============================================================================
*/
fn main() {
  let input = include_str!("./input.txt");
  let answer = run(&input);

  println!("Answer:");
  println!("{answer:?}");
  println!("================================================");
  println!("Input:");
  println!("{input}");
  println!("================================================");
}

#[cfg(test)]
mod test {
  use test_case::test_case;

  #[test_case(vec![Some('A'), None], "[A]     \n 1   2  ")]
  #[test_case(vec![None, Some('B')], "    [B] \n 1   2  ")]
  fn one_row_without_actions(expected: Vec<Option<char>>, input: &str) {
    assert_eq!(expected, crate::run(&input));
  }
}