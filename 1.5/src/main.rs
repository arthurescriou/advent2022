use std::env;
use std::fs;

fn main() {
    let file_path = "./input";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut lists = contents.trim().split("\n\n").map(|x| count(x)).collect::<Vec<u32>>();
    lists.sort();
    let rev = lists.into_iter().rev().collect::<Vec<u32>>();
    println!("{}", rev[0]+rev[1]+rev[2]);

}

fn count(str: &str)-> u32 {
    return str.split("\n").map(|x| x.parse::<u32>().unwrap()).sum();
}
