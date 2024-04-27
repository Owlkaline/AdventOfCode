use std::fs;

fn main() {
  // Part 1
  //
  //  let mut fully_contained_range = 0;
  //
  //  if let Ok(contents) = fs::read_to_string("pairs") {
  //    for line in contents.lines() {
  //      let pair: Vec<String> = line.split(',').map(|a| a.to_string()).collect();
  //      let mut a: Vec<i32> = pair[0]
  //        .split('-')
  //        .map(|a| a.parse::<i32>().unwrap())
  //        .collect();
  //      let mut b: Vec<i32> = pair[1]
  //        .split('-')
  //        .map(|a| a.parse::<i32>().unwrap())
  //        .collect();
  //
  //      println!("a {:?} b {:?}", a, b);
  //
  //      if a[1] - a[0] > b[1] - b[0] {
  //        let t = a;
  //        a = b;
  //        b = t;
  //      }
  //
  //      let mut range_contained = true;
  //      for i in a[0]..(a[1] + 1) {
  //        println!("i {} {}..{}", i, b[0], b[1]);
  //        if !(b[0]..(b[1] + 1)).contains(&i) {
  //          println!("not contained");
  //          range_contained = false;
  //          break;
  //        }
  //      }
  //      if range_contained {
  //        fully_contained_range += 1;
  //      }
  //    }
  //  }
  //
  //  println!("Ranges that are fully contained: {}", fully_contained_range);

  // Part 2
  let mut overlaps = 0;

  if let Ok(contents) = fs::read_to_string("pairs") {
    for line in contents.lines() {
      let pair: Vec<String> = line.split(',').map(|a| a.to_string()).collect();
      let mut a: Vec<i32> = pair[0]
        .split('-')
        .map(|a| a.parse::<i32>().unwrap())
        .collect();
      let mut b: Vec<i32> = pair[1]
        .split('-')
        .map(|a| a.parse::<i32>().unwrap())
        .collect();

      let mut range_overlap = false;
      for i in a[0]..(a[1] + 1) {
        if (b[0]..(b[1] + 1)).contains(&i) {
          range_overlap = true;
          break;
        }
      }
      if range_overlap {
        overlaps += 1;
      }
    }
  }

  println!("Total number of assignment overlaps are: {}", overlaps);
}
