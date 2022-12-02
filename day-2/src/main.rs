use core::panic;
use std::fs::File;
use std::io::prelude::*;

fn main() {
  let result = play(&read_file("input.txt"));
  println!("final score: {result}");
}

fn read_file(path: &str) -> String {
  let mut error_msg: String = String::from("error reading file: ");
  error_msg.push_str(&path);

  let mut file: File = File::open(path).expect(&error_msg);

  let mut contents: String = String::new();
  file.read_to_string(&mut contents).expect(&error_msg);

  contents
}

fn play(contents: &String) -> u32 {
  let mut sum: u32 = 0;
  for line in contents.lines() {
    let opponent = line.chars().nth(0).unwrap();
    let me = line.chars().nth(2).unwrap();
    let score = check_outcome(opponent, me);
    sum += score;
  }
  sum
}

fn check_outcome(opponent: char, me: char) -> u32 {
  let mut score: u32 = 0;
  match me {
    'X' => score += 0,
    'Y' => score += 3,
    'Z' => score += 6,
    _ => panic!("unexpreced input"),
  }
  match opponent {
    'A' => match me {
      'X' => score += 3,
      'Y' => score += 1,
      'Z' => score += 2,
      _ => panic!("unexpreced input"),
    },
    'B' => match me {
      'X' => score += 1,
      'Y' => score += 2,
      'Z' => score += 3,
      _ => panic!("unexpreced input"),
    },
    'C' => match me {
      'X' => score += 2,
      'Y' => score += 3,
      'Z' => score += 1,
      _ => panic!("unexpreced input"),
    },
    _ => panic!("unexpreced input"),
  }
  println!("{opponent} {me} {score}");
  score
}
