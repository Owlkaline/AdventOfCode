use std::fs;

fn main() {
  println!("Hello, world!");

  let carried_calories = "elf_food";

  // Setup first elf with 0 calories
  let mut elves = vec![0];

  if let Ok(contents) = fs::read_to_string(carried_calories) {
    for line in contents.lines() {
      if line.len() == 0 {
        // Move onto next elf
        elves.push(0);
      } else {
        // parse line into calories
        // add it to the total calory count for current elf

        let item_calory = line.parse::<i32>().unwrap();

        if let Some(current_elf) = elves.last_mut() {
          *current_elf += item_calory;
        }
      }
    }
  }

  elves.sort_by(|a, b| b.cmp(a));

  println!("Highest calory count any single elf has is: {}", elves[0]);
  println!(
    "Total calories carried by top 3 elves: {}",
    elves[0..3].iter().sum::<i32>()
  );
}
