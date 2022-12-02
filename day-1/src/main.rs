use std::fs::File;
use std::io::prelude::*;

fn main() {
  println!("start");
  let input: String = read_file("input.txt");
  let calories: Vec<u32> = elf_calories(input);
  let (max_elf, max_calories): (usize, u32) = find_most_calories(&calories);
  println!("elf {max_elf} has the most calories with: {max_calories}\n");
  let top: (Vec<(u16, u32)>, u32) = output_top_calories(&calories, 3);
  for (index, value) in top.0.iter().enumerate() {
    println!(
      "elf {} has the {}. most calories with: {}\n",
      value.0,
      index + 1,
      value.1
    );
  }
  println!("totaling: {} calories", top.1);
  println!("end");
}

fn read_file(path: &str) -> String {
  let mut error_msg: String = String::from("error reading file: ");
  error_msg.push_str(&path);

  let mut file: File = File::open(path).expect(&error_msg);

  let mut contents: String = String::new();
  file.read_to_string(&mut contents).expect(&error_msg);

  contents
}

fn elf_calories(input: String) -> Vec<u32> {
  let mut calories: Vec<u32> = Vec::new();
  let mut current_elf: usize = 0;
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
  calories
}

fn find_most_calories(calories: &Vec<u32>) -> (usize, u32) {
  let mut max_elf: usize = 0;
  let mut max_calories: u32 = 0;
  for (curr_elf, curr_calories) in calories.iter().enumerate() {
    if curr_calories > &max_calories {
      max_elf = curr_elf;
      max_calories = *curr_calories;
    }
  }

  (max_elf, max_calories)
}

fn output_top_calories(calories: &Vec<u32>, count: usize) -> (Vec<(u16, u32)>, u32) {
  let mut top: Vec<(u16, u32)> = Vec::new();
  for _ in 0..count {
    top.push((0, 0));
  }
  for (curr_elf, curr_calories) in calories.iter().enumerate() {
    for i in 0..count {
      if *curr_calories > top[i].1 {
        for j in (i + 1..count).rev() {
          top[j] = top[j - 1];
        }
        top[i] = (curr_elf as u16, *curr_calories);
        break;
      }
    }
  }
  let total: u32 = top.iter().map(|&(_, cals)| cals).sum();
  (top, total)
}
