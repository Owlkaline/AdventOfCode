use std::fs;

use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Position {
  x: i32,
  y: i32,
}

impl Position {
  pub fn new(x: i32, y: i32) -> Position {
    Position { x, y }
  }

  fn neigbours_locations(&self) -> Vec<Position> {
    let mut locations = Vec::new();
    for i in 0..3 {
      let y = self.y + i as i32 - 1;
      if y >= 0 {
        for j in 0..3 {
          let x = self.x + j as i32 - 1;
          //println!("x {} y {}", x, y);
          if x >= 0 {
            locations.push(Position::new(x, y));
          }
        }
      }
    }

    locations
  }
}

fn main() {
  println!("Hello, world!");
  part_one("full");
  part_two("full");
}

fn load_map(filename: &str) -> (i32, i32, HashMap<Position, String>) {
  let mut map = HashMap::new();

  let mut x = 0;
  let mut y = 0;
  let mut max_x = 0;
  if let Ok(contents) = fs::read_to_string(filename) {
    for line in contents.lines() {
      let parts: Vec<String> = line.chars().map(|a| a.to_string()).collect();
      for entry in parts {
        map.insert(Position::new(x, y), entry);
        x += 1;
        max_x = max_x.max(x);
      }
      x = 0;
      y += 1;
    }
  }

  (max_x, y, map)
}

fn part_two(filename: &str) {
  let (max_x, y, map) = load_map(filename);

  let mut symbol_map: HashMap<&String, Vec<Vec<u32>>> = HashMap::new();

  for i in 0..y {
    for j in 0..max_x {
      if let Some(value) = map.get(&Position::new(j, i)) {
        if *value != "." && value.parse::<u32>().is_err() {
          // This is a symbol
          let symbol_position = Position::new(j, i);
          let symbol = map.get(&symbol_position).unwrap();
          let values = get_neigbouring_values(symbol_position, &map, max_x);

          if let Some(existing_symbol_values) = symbol_map.get_mut(symbol) {
            existing_symbol_values.push(values);
          } else {
            symbol_map.insert(symbol, vec![values]);
          }
        }
      }
    }
  }

  let mut total_value = 0;
  if let Some(gears) = symbol_map.get(&"*".to_string()) {
    for gear_parts in gears {
      if gear_parts.len() == 2 {
        let gear_ratio = gear_parts[0] * gear_parts[1];
        total_value += gear_ratio;
      }
    }
  }

  println!("total: {}", total_value);
}

fn part_one(filename: &str) {
  let (max_x, y, map) = load_map(filename);

  let mut all_numbers_to_add = Vec::new();

  for i in 0..y {
    for j in 0..max_x {
      if let Some(value) = map.get(&Position::new(j, i)) {
        if *value != "." && value.parse::<u32>().is_err() {
          // This is a symbol
          let symbol_position = Position::new(j, i);
          let mut values = get_neigbouring_values(symbol_position, &map, max_x);

          all_numbers_to_add.append(&mut values);
        }
      }
    }
  }

  let total_value = all_numbers_to_add.iter().sum::<u32>();
  println!("Sum of all part numbers: {}", total_value);
}

fn get_neigbouring_values(
  position: Position,
  map: &HashMap<Position, String>,
  max_x: i32,
) -> Vec<u32> {
  let mut all_neighbour_values = Vec::new();

  let neigbours = position.neigbours_locations();

  let mut positions_found = Vec::new();

  for neigbour in neigbours {
    if positions_found.contains(&neigbour) {
      continue;
    }

    if let Some(value) = map.get(&neigbour) {
      if let Ok(adjecent_value) = value.parse::<u32>() {
        positions_found.push(neigbour);
        // search left and right of this value toget whole number

        let mut numbers_before = Vec::new();
        let mut numbers_after = Vec::new();

        // search left
        let mut neigbour_x = neigbour.x - 1;
        while neigbour_x >= 0 {
          if let Some(value) = map.get(&Position::new(neigbour_x, neigbour.y)) {
            if let Ok(value_before) = value.parse::<u32>() {
              positions_found.push(Position::new(neigbour_x, neigbour.y));
              numbers_before.push(value_before);
            } else {
              neigbour_x = -1;
            }
          }
          neigbour_x -= 1;
        }

        // make it in order of left to right
        numbers_before.reverse();
        numbers_before.push(adjecent_value);

        // search right
        neigbour_x = neigbour.x + 1;
        while neigbour_x < max_x + 1 {
          if let Some(value) = map.get(&Position::new(neigbour_x, neigbour.y)) {
            if let Ok(value_after) = value.parse::<u32>() {
              positions_found.push(Position::new(neigbour_x, neigbour.y));
              numbers_after.push(value_after);
            } else {
              neigbour_x = max_x + 1;
            }
          }
          neigbour_x += 1;
        }

        let mut whole_number = Vec::new();
        whole_number.append(&mut numbers_before);
        whole_number.append(&mut numbers_after);

        let mut value_string = "".to_string();
        for num in whole_number.iter() {
          value_string = format!("{}{}", value_string, num);
        }

        let number = value_string.parse::<u32>().unwrap();
        all_neighbour_values.push(number);
      }
    }
  }

  all_neighbour_values
}
