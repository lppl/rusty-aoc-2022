#[derive(Debug, Clone, Copy)]
enum Outcome {
  Win,
  Draw,
  Lose,
}

impl Outcome {
  fn points(self) -> usize {
    match self {
      Outcome::Win => 6,
      Outcome::Draw => 3,
      Outcome::Lose => 0,
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Hand {
  Rock,
  Paper,
  Scissors,
}

impl Hand {
  fn hand_score(self) -> usize {
    match self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3,
    }
  }

  fn beats(self, theirs: Hand) -> bool {
    matches!(
      (self, theirs),
      (Self::Rock, Self::Scissors) | (Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper)
    )
  }

  fn outcome(self, theirs: Hand) -> Outcome {
    if self.beats(theirs) {
      Outcome::Win
    } else if theirs.beats(self) {
      Outcome::Lose
    } else {
      Outcome::Draw
    }
  }

  fn outcome_score(self, theirs: Hand) -> usize {
    self.outcome(theirs).points()
  }
}

impl TryFrom<char> for Hand {
  type Error = color_eyre::Report;

  fn try_from(c: char) -> Result<Self, Self::Error> {
    match c {
      'A' | 'X' => Ok(Hand::Rock),
      'B' | 'Y' => Ok(Hand::Paper),
      'C' | 'Z' => Ok(Hand::Scissors),
      _ =>
        Err(
          color_eyre::eyre::eyre!(
            "Invalid hand. Given={c:?}. Should be 'A', 'X', 'B', 'Y', 'C', or 'Z"
          )
        ),
    }
  }
}

#[derive(Debug, Clone, Copy)]
struct Round {
  ours: Hand,
  theirs: Hand,
}

impl Round {
  fn score(self) -> usize {
    self.ours.hand_score() + self.ours.outcome_score(self.theirs)
  }
}

impl std::str::FromStr for Round {
  type Err = color_eyre::Report;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut chars = s.chars();
    let (Some(theirs), Some(' '), Some(ours), None) = (
      chars.next(),
      chars.next(),
      chars.next(),
      chars.next(),
    ) else {
      return Err(color_eyre::eyre::eyre!("expected <theirs>space<theirs>EOF, got {s:?}"));
    };

    Ok(Self {
      ours: ours.try_into()?,
      theirs: theirs.try_into()?,
    })
  }
}

pub fn rate_game(s: &str) -> i32 {
  String::from(s)
    .lines()
    .map(|line| line.parse::<Round>())
    .map(|round| {
      match round {
        Ok(round) => round.score() as i32,
        Err(err) => panic!("{}", err),
      }
    })
    .sum()
}