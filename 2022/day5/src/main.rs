use std::collections::HashMap;
use std::fs;

fn main() {
  // part 1
  //if let Ok(contents) = fs::read_to_string("box_stacking") {
  //  let mut columns: i32 = -1;

  //  let mut boxes: HashMap<i32, Vec<char>> = HashMap::new();
  //  let mut finished_collecting_box_data = false;

  //  for line in contents.lines() {
  //    if columns == -1 {
  //      columns = (line.len() / 4) as i32 + 1;
  //    }

  //    if line.contains("move") {
  //      if !finished_collecting_box_data {
  //        for i in 0..columns + 1 {
  //          if let Some(column) = boxes.get_mut(&i) {
  //            column.reverse();
  //          }
  //        }
  //        finished_collecting_box_data = true;
  //      }

  //      let box_moves: Vec<_> = line
  //        .split(' ')
  //        .filter_map(|a| {
  //          if let Ok(a) = a.parse::<i32>() {
  //            Some(a)
  //          } else {
  //            None
  //          }
  //        })
  //        .collect();

  //      let number_of_boxes = box_moves[0];
  //      let from_column = box_moves[1];
  //      let to_column = box_moves[2];
  //      println!("box_moves: {:?}", box_moves);

  //      let mut moving_boxes = Vec::new();
  //      if let Some(stack) = boxes.get_mut(&from_column) {
  //        for i in 0..number_of_boxes {
  //          moving_boxes.push(
  //            stack
  //              .pop()
  //              .unwrap()
  //              .to_string()
  //              .chars()
  //              .collect::<Vec<char>>()[0],
  //          );
  //        }
  //      }
  //      if let Some(stack) = boxes.get_mut(&to_column) {
  //        for i in 0..number_of_boxes as usize {
  //          stack.push(moving_boxes[i]);
  //        }
  //      }

  //      for i in 0..columns + 1 {
  //        if let Some(column) = boxes.get_mut(&i) {
  //          println!("column {}: {:?}", i, column);
  //        }
  //      }
  //      continue;
  //    }

  //    let box_indices: Vec<_> = line.match_indices('[').map(|(i, _)| i).collect();
  //    let box_letters: Vec<_> = line
  //      .split(' ')
  //      .filter_map(|a| {
  //        if a.is_empty() {
  //          None
  //        } else {
  //          Some(a.to_string())
  //        }
  //      })
  //      .collect();

  //    for (idx, box_idx) in box_indices.iter().enumerate() {
  //      let column_number = (box_idx / 4) as i32 + 1;
  //      let box_letter = box_letters[idx].chars().into_iter().collect::<Vec<char>>()[1];

  //      if boxes.contains_key(&column_number) {
  //        if let Some(column) = boxes.get_mut(&column_number) {
  //          column.push(box_letter);
  //        }
  //      } else {
  //        boxes.insert(column_number, vec![box_letter]);
  //      }
  //    }
  //  }

  //  for i in 0..columns + 1 {
  //    if let Some(column) = boxes.get_mut(&i) {
  //      //    column.reverse();
  //      //  println!("column {}: {:?}", i, column);
  //      if let Some(letter) = column.last() {
  //        print!("{}", letter);
  //      }
  //    }
  //  }
  //  println!("");
  //}

  // Part 2
  if let Ok(contents) = fs::read_to_string("box_stacking") {
    let mut columns: i32 = -1;

    let mut boxes: HashMap<i32, Vec<char>> = HashMap::new();
    let mut finished_collecting_box_data = false;

    for line in contents.lines() {
      if columns == -1 {
        columns = (line.len() / 4) as i32 + 1;
      }

      if line.contains("move") {
        if !finished_collecting_box_data {
          for i in 0..columns + 1 {
            if let Some(column) = boxes.get_mut(&i) {
              column.reverse();
            }
          }
          finished_collecting_box_data = true;
        }

        let box_moves: Vec<_> = line
          .split(' ')
          .filter_map(|a| {
            if let Ok(a) = a.parse::<i32>() {
              Some(a)
            } else {
              None
            }
          })
          .collect();

        let number_of_boxes = box_moves[0];
        let from_column = box_moves[1];
        let to_column = box_moves[2];
        println!("box_moves: {:?}", box_moves);

        let mut moving_boxes = Vec::new();
        if let Some(stack) = boxes.get_mut(&from_column) {
          for i in 0..number_of_boxes {
            moving_boxes.push(
              stack
                .pop()
                .unwrap()
                .to_string()
                .chars()
                .collect::<Vec<char>>()[0],
            );
          }
        }
        if let Some(stack) = boxes.get_mut(&to_column) {
          for i in (0..number_of_boxes as usize).rev() {
            stack.push(moving_boxes[i]);
          }
        }

        for i in 0..columns + 1 {
          if let Some(column) = boxes.get_mut(&i) {
            println!("column {}: {:?}", i, column);
          }
        }
        continue;
      }

      let box_indices: Vec<_> = line.match_indices('[').map(|(i, _)| i).collect();
      let box_letters: Vec<_> = line
        .split(' ')
        .filter_map(|a| {
          if a.is_empty() {
            None
          } else {
            Some(a.to_string())
          }
        })
        .collect();

      for (idx, box_idx) in box_indices.iter().enumerate() {
        let column_number = (box_idx / 4) as i32 + 1;
        let box_letter = box_letters[idx].chars().into_iter().collect::<Vec<char>>()[1];

        if boxes.contains_key(&column_number) {
          if let Some(column) = boxes.get_mut(&column_number) {
            column.push(box_letter);
          }
        } else {
          boxes.insert(column_number, vec![box_letter]);
        }
      }
    }

    for i in 0..columns + 1 {
      if let Some(column) = boxes.get_mut(&i) {
        //    column.reverse();
        //  println!("column {}: {:?}", i, column);
        if let Some(letter) = column.last() {
          print!("{}", letter);
        }
      }
    }
    println!("");
  }
}
