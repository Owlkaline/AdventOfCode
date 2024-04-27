use std::fs;

fn main() {
  // part 1
  //let mut priority_value = 0;

  //if let Ok(contents) = fs::read_to_string("rucksack") {
  //  for line in contents.lines() {
  //    let (one, two) = line.split_at(line.len() / 2);

  //    let mut similar_character = None;
  //    for a in one.chars() {
  //      for b in two.chars() {
  //        if a == b {
  //          similar_character = Some(a);
  //        }
  //      }
  //    }

  //    if let Some(c) = similar_character {
  //      let value = if c.is_lowercase() {
  //        c as i32 - 'a' as i32 + 1
  //      } else {
  //        c as i32 - 'A' as i32 + 27
  //      };

  //      priority_value += value;
  //    }
  //  }
  //}

  //println!("Total priority is {}", priority_value);

  // Part 2

  let mut priority_value = 0;

  if let Ok(contents) = fs::read_to_string("rucksack") {
    for packs in contents
      .lines()
      .map(|a| a.to_string())
      .collect::<Vec<String>>()
      .chunks(3)
    {
      //let (one, two) = line.split_at(line.len() / 2);
      let p1 = &packs[0];
      let p2 = &packs[1];
      let p3 = &packs[2];

      let mut similar_character = None;
      for a in p1.chars() {
        for b in p2.chars() {
          if a == b {
            for c in p3.chars() {
              if b == c {
                similar_character = Some(a);
              }
            }
          }
        }
      }

      if let Some(c) = similar_character {
        let value = if c.is_lowercase() {
          c as i32 - 'a' as i32 + 1
        } else {
          c as i32 - 'A' as i32 + 27
        };

        priority_value += value;
      }
    }
  }

  println!("Priority of badges {}", priority_value);
}
