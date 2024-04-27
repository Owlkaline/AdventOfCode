use std::fs;

#[derive(PartialEq)]
pub enum Outcome {
  Win,
  Draw,
  Lose,
}

pub enum RPSShape {
  Rock,
  Paper,
  Scissors,
}

impl Outcome {
  pub fn points(&self) -> i32 {
    match *self {
      Outcome::Lose => 0,
      Outcome::Draw => 3,
      Outcome::Win => 6,
    }
  }

  pub fn convert(encprypted_move: &str) -> Outcome {
    match encprypted_move {
      "X" => Outcome::Lose,
      "Y" => Outcome::Draw,
      "Z" => Outcome::Win,
      _ => Outcome::Draw,
    }
  }
}

impl RPSShape {
  pub fn all() -> Vec<RPSShape> {
    vec![RPSShape::Rock, RPSShape::Paper, RPSShape::Scissors]
  }

  pub fn convert(encprypted_move: &str) -> RPSShape {
    match encprypted_move {
      "A" | "X" => RPSShape::Rock,
      "B" | "Y" => RPSShape::Paper,
      "C" | "Z" => RPSShape::Scissors,
      _ => RPSShape::Rock,
    }
  }

  pub fn compare_shapes(&self, other: &RPSShape) -> Outcome {
    match (self, other) {
      (RPSShape::Rock, RPSShape::Paper)
      | (RPSShape::Paper, RPSShape::Scissors)
      | (RPSShape::Scissors, RPSShape::Rock) => Outcome::Lose,
      (RPSShape::Paper, RPSShape::Rock)
      | (RPSShape::Rock, RPSShape::Scissors)
      | (RPSShape::Scissors, RPSShape::Paper) => Outcome::Win,
      _ => Outcome::Draw,
    }
  }

  pub fn calculate_required_shape_for_outcome(&self, desired_outcome: &Outcome) -> RPSShape {
    let mut shape_required = RPSShape::Rock;

    for shape in RPSShape::all() {
      if shape.compare_shapes(self) == *desired_outcome {
        shape_required = shape;
        break;
      }
    }

    shape_required
  }

  pub fn points(&self) -> i32 {
    match *self {
      RPSShape::Rock => 1,
      RPSShape::Paper => 2,
      RPSShape::Scissors => 3,
    }
  }
}

fn main() {
  let mut total_points = 0;

  // Part 1
  //if let Ok(contents) = fs::read_to_string("rps_strat") {
  //  for line in contents.lines() {
  //    let strategy = line.split(' ').collect::<Vec<&str>>();

  //    let them = RPSShape::convert(strategy[0]);
  //    let us = RPSShape::convert(strategy[1]);

  //    let result = us.compare_shapes(&them);

  //    let set_total = us.points() + result.points();
  //    total_points += set_total;
  //  }
  //}

  //println!("Overall points: {}", total_points);
  //

  // Part 2
  if let Ok(contents) = fs::read_to_string("rps_strat") {
    for line in contents.lines() {
      let strategy = line.split(' ').collect::<Vec<&str>>();

      let them = RPSShape::convert(strategy[0]);
      let result = Outcome::convert(strategy[1]);

      let us = them.calculate_required_shape_for_outcome(&result);

      let set_total = us.points() + result.points();
      total_points += set_total;
    }
  }

  println!("Overall points: {}", total_points);
}
