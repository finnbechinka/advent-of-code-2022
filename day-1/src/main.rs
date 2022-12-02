use std::fs::File;
use std::io::prelude::*;

fn main() {
  println!("start");
  let input = read_file("input.txt");
  let calories = elf_calories(input);
  output_most_calories(&calories);
  output_top3_calories(&calories);
  println!("end");
}

fn read_file(path: &str) -> String {
  let mut error_msg = String::from("error reading file: ");
  error_msg.push_str(&path);

  let mut file = File::open(path).expect(&error_msg);

  let mut contents = String::new();
  file.read_to_string(&mut contents).expect(&error_msg);

  return contents;
}

fn elf_calories(input: String) -> Vec<u32> {
  let mut calories = Vec::new();
  let mut current_elf = 0;
  for line in input.lines() {
    if line == "" {
      current_elf = current_elf + 1;
    } else {
      let line_value: u32 = line.parse().unwrap();
      match calories.get(current_elf) {
        Some(current_calories) => {
          let new_calories = current_calories + line_value;
          calories[current_elf] = new_calories;
        }
        None => calories.push(line_value),
      }
    }
  }
  return calories;
}

fn output_most_calories(calories: &Vec<u32>) {
  let mut max_elf: usize = 0;
  let mut max_calories: u32 = 0;
  for (curr_elf, curr_calories) in calories.iter().enumerate() {
    if curr_calories > &max_calories {
      max_elf = curr_elf;
      max_calories = *curr_calories;
    }
  }
  println!("elf {max_elf} has the most calories with: {max_calories}\n");
}

fn output_top3_calories(calories: &Vec<u32>) {
  let mut top3_elfs = [0, 0, 0];
  let mut top3_calories = [0, 0, 0];
  for (curr_elf, curr_calories) in calories.iter().enumerate() {
    if curr_calories > &top3_calories[0] {
      top3_elfs[2] = top3_elfs[1];
      top3_elfs[1] = top3_elfs[0];
      top3_elfs[0] = curr_elf;

      top3_calories[2] = top3_calories[1];
      top3_calories[1] = top3_calories[0];
      top3_calories[0] = *curr_calories;
    } else if curr_calories > &top3_calories[1] {
      top3_elfs[2] = top3_elfs[1];
      top3_elfs[1] = curr_elf;

      top3_calories[2] = top3_calories[1];
      top3_calories[1] = *curr_calories;
    } else if curr_calories > &top3_calories[2] {
      top3_elfs[2] = curr_elf;

      top3_calories[2] = *curr_calories;
    }
  }

  println!(
    "elf {} has the third most calories with: {}",
    top3_elfs[2], top3_calories[2]
  );
  println!(
    "elf {} has the second most calories with: {}",
    top3_elfs[1], top3_calories[1]
  );
  println!(
    "elf {} has the most calories with: {}",
    top3_elfs[0], top3_calories[0]
  );

  println!(
    "totaling: {}",
    top3_calories[0] + top3_calories[1] + top3_calories[2]
  );
}
