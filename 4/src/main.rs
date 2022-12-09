use std::env;
use std::fs;

#[derive(Copy, Clone)]
struct Assignement {
  start: u32,
  end: u32,
}

#[derive(Copy, Clone)]
struct Pair {
  first: Assignement,
  second: Assignement,
}

fn main() {
  let file_path = "./input";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let res: usize = contents
    .trim()
    .split("\n")
    .map(|str| {
      let parsed: Vec<Assignement> = str
        .split(",")
        .map(|assign_str| {
          let hours: Vec<&str> = assign_str.split("-").collect();
          let start: u32 = hours[0].parse().unwrap();
          let end: u32 = hours[1].parse().unwrap();
          return Assignement {
            start: start,
            end: end,
          };
        })
        .collect();
      return Pair {
        first: parsed[0],
        second: parsed[1],
      };
    })
    .filter(|p| overlap(*p))
    .count();
  println!("{}", res)
}

fn overlap(pair: Pair) -> bool {
  if pair.first.start == pair.second.start {
    return true;
  }
  if pair.first.start < pair.second.start {
    return pair.first.end >= pair.second.start;
  }
  return pair.first.start <= pair.second.end;
}
