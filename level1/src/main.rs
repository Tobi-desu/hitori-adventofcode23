use std::fs;

fn main() {
    let amongus = fs::read_to_string("./src/input.txt").expect("Couldn't open file");
    part1(amongus);
}

fn part1(amongus: String) {
    let mut vec: Vec<u32> = Vec::new();
    for line in amongus.lines() {
        let mut first: char = '0';
        let mut last: char = '0';
        for c in line.chars() {
            if c.is_digit(10) {
                first = c;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last = c;
                break;
            }
        }
        let add_num: u32 = format!("{first}{last}").parse::<u32>().unwrap();
        vec.push(add_num);
    }
    for val in vec.iter() {
        println!("{}", val)
    }
    let sum: u32 = vec.iter().sum();
    println!("{}", sum);
}


//finishing later
fn part2(amongus: String){
    let num_names = ("one", "two", "three", "four", "five", "six", "seven", "eight", "nine");
    let mut vec: Vec<u32> = Vec::new();
    for line in amongus.lines(){
        let mut first: &str = "0";
        let mut last: &str = "0";

    }
}
