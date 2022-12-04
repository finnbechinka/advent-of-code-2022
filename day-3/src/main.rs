use std::{fs::File, io::Read};

fn main() {
  let input = read_file("input.txt");
  let rucksacks = parse_rucksacks(&input);
  let duplicates = find_duplicates(&rucksacks);
  let result = calculate_priority_sum(duplicates);
  println!("{result}");
  let badges = determine_group_types(&rucksacks);
  let result = calculate_priority_sum(badges);
  println!("{result}");
}

fn read_file(path: &str) -> String {
  let mut error_msg: String = String::from("error reading file: ");
  error_msg.push_str(&path);

  let mut file: File = File::open(path).expect(&error_msg);

  let mut contents: String = String::new();
  file.read_to_string(&mut contents).expect(&error_msg);

  contents
}

fn parse_rucksacks(input: &str) -> Vec<(&str, &str)> {
  let mut rucksacks = Vec::new();
  for line in input.lines() {
    rucksacks.push(line.split_at(line.len() / 2));
  }
  rucksacks
}

fn find_duplicates(rucksacks: &Vec<(&str, &str)>) -> Vec<char> {
  let mut duplicates = Vec::new();
  for rucksack in rucksacks {
    let mut ignore = Vec::new();
    for item in rucksack.0.chars() {
      if ignore.contains(&item) {
        continue;
      }
      if rucksack.1.contains(item) {
        ignore.push(item);
        duplicates.push(item);
      }
    }
  }
  return duplicates;
}

fn calculate_priority_sum(items: Vec<char>) -> u32 {
  let mut lookup_table = ('a'..='z').collect::<Vec<char>>();
  lookup_table.extend(('A'..='Z').collect::<Vec<char>>());
  let mut sum = 0;
  for item in items {
    sum += 1 + lookup_table.iter().position(|c| c == &item).unwrap() as u32;
  }
  sum
}

fn determine_group_types(rucksacks: &Vec<(&str, &str)>) -> Vec<char> {
  let mut badges = Vec::new();
  for (index, _) in rucksacks.iter().enumerate().step_by(3) {
    let group = [
      format!("{}{}", &rucksacks[index].0, &rucksacks[index].1),
      format!("{}{}", &rucksacks[index + 1].0, &rucksacks[index + 1].1),
      format!("{}{}", &rucksacks[index + 2].0, &rucksacks[index + 2].1),
    ];
    for item in group[0].chars() {
      if group[1].contains(item) {
        if group[2].contains(item) {
          badges.push(item);
          break;
        }
      }
    }
  }
  badges
}
