use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let filename = "input.txt";
    let content = get_content(filename);

    //let result = part1(content);
    let result = part2(content);
    println!("{}", result);
}

fn get_content(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Cannot find file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("Cannot read input file");

    let content: Vec<String> = file_contents.split("\n")
        .map(|s: &str|s.to_string())
        .collect();

    content
}

fn parse_line(line: &str) -> i32 {
    line.trim_left_matches('+').parse().unwrap()

    // let mut result: Result<String, String> = match line.trim_left_matches('+').parse() {
    //     Ok(n) => n,
    //     Err(error) => match error.kind() {
    //         ErrorKind::ParseIntError => {
    //             println!("Could not parse {}", line);
    //             0
    //         }
    //     }
    // };

    // result.unwrap()
}

fn part1(lines: Vec<String>) -> i32 {
    let mut frequency = 0;
    for line in lines {
        let frequency_change:i32 = parse_line(&line);
        frequency += frequency_change;
    }

    frequency
}

fn part2(lines: Vec<String>) -> i32 {
    let mut frequency = 0;
    for line in lines {
        let frequency_change:i32 = parse_line(&line);
        frequency += frequency_change;
    }

    frequency
}
