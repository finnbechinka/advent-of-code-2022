use std::{fs::File, io::Read};

fn main() {
  let input = read_file("input.txt");
  let pairs = parse_pairs(&input);
  let count = count_fully_contained(&pairs);
  println!("{}", count);
}

fn read_file(path: &str) -> String {
  let mut error_msg: String = String::from("error reading file: ");
  error_msg.push_str(&path);

  let mut file: File = File::open(path).expect(&error_msg);

  let mut contents: String = String::new();
  file.read_to_string(&mut contents).expect(&error_msg);

  contents
}

fn parse_pairs(input: &String) -> Vec<(Vec<u16>, Vec<u16>)> {
  let mut pairs = Vec::new();

  for line in input.lines() {
    let str_ranges = line.split(",").collect::<Vec<&str>>();
    let mut pair_bounds = [Vec::new(), Vec::new()];
    for (i, str) in str_ranges.iter().enumerate() {
      let bounds = str
        .split("-")
        .map(|bound| bound.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();
      pair_bounds[i] = bounds;
    }
    pairs.push((pair_bounds[0].clone(), pair_bounds[1].clone()));
  }
  pairs
}

fn count_fully_contained(pairs: &Vec<(Vec<u16>, Vec<u16>)>) -> u32 {
  let mut count = 0;
  for pair in pairs {
    let a_contains_b = pair.0.get(0).unwrap() <= pair.1.get(0).unwrap()
      && pair.0.get(1).unwrap() >= pair.1.get(1).unwrap();
    let b_contains_a = pair.1.get(0).unwrap() <= pair.0.get(0).unwrap()
      && pair.1.get(1).unwrap() >= pair.0.get(1).unwrap();
    if a_contains_b || b_contains_a {
      count += 1;
    }
  }
  count
}
