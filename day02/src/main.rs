#[derive(Clone, Copy, PartialEq)]
enum Rpc {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

#[derive(PartialEq)]
enum Points {
  Lose = 0,
  Draw = 3,
  Win = 6,
}

fn them_points(s: &str) -> Rpc {
  match s {
    "A" => Rpc::Rock,
    "B" => Rpc::Paper,
    "C" => Rpc::Scissors,
    _ => panic!("Invalid input, should be one of: A, B, C"),
  }
}

fn me_points(s: &str) -> Rpc {
  match s {
    "X" => Rpc::Rock,
    "Y" => Rpc::Paper,
    "Z" => Rpc::Scissors,
    _ => panic!("Invalid input, should be one of: X, Y, Z"),
  }
}

fn rate_round(them: Rpc, me: Rpc) -> i32 {
  let win = match (them, me) {
    (Rpc::Rock, Rpc::Paper) => Points::Win as i32,
    (Rpc::Paper, Rpc::Scissors) => Points::Win as i32,
    (Rpc::Scissors, Rpc::Rock) => Points::Win as i32,
    _ => Points::Lose as i32,
  };

  let draw = if them == me { Points::Draw as i32 } else { Points::Lose as i32 };

  return win + draw + (me as i32);
}

fn rate_game(s: &str) -> i32 {
  s.lines()
    .map(|line| line.split_once(' ').unwrap())
    .map(|(them, me)| (them_points(them), me_points(me)))
    .map(|(them, me)| rate_round(them, me))
    .sum::<i32>()
}

fn main() {
  assert_eq!(4, rate_game("A X"), "draw using rocks");
  assert_eq!(5, rate_game("B Y"), "draw using papers");
  assert_eq!(6, rate_game("C Z"), "draw using scissors");

  assert_eq!(7, rate_game("C X"), "winning using rocks");
  assert_eq!(8, rate_game("A Y"), "winning using papers");
  assert_eq!(9, rate_game("B Z"), "winning using scissors");

  assert_eq!(1, rate_game("B X"), "losing using rocks");
  assert_eq!(2, rate_game("C Y"), "losing using papers");
  assert_eq!(3, rate_game("A Z"), "losing using scissors");

  assert_eq!(15, rate_game("A Y\nB X\nC Z"), "game for 15 points");
}