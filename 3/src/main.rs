use std::env;
use std::fs;

fn main() {
  let file_path = "./input";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

  let lists: u32 = contents
    .trim()
    .split("\n")
    .map(|x| {
      [&x[0..x.len() / 2], &x[x.len() / 2..x.len()]]
        .into_iter()
        .collect()
    })
    .map(|tab| get_val(tab))
    .sum();
  let res = lists;
  println!("{}", res)
}

fn letter_to_val(letter: char) -> u32 {
  let code = letter as u32;
  let a = 'a' as u32;
  if code < a {
    return code - 'A' as u32 as u32 + 27;
  }
  return code - 'a' as u32 + 1;
}

fn get_val(tab: Vec<&str>) -> u32 {
  let p1 = tab[0];
  let p2 = tab[1];
  let letter = p1.chars().fold('a', |acc, val| {
    if p2.chars().position(|c| c == val) == None {
      acc
    } else {
      val
    }
  });
  return letter_to_val(letter);
}
