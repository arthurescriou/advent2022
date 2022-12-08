use std::env;
use std::fs;

fn main() {
    let file_path = "./input";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lists = contents.trim().split("\n\n").map(|x| count(x)).max().unwrap();
    println!("{}", lists);

}

fn count(str: &str)-> u32 {
    return str.split("\n").map(|x| x.parse::<u32>().unwrap()).sum();
}
