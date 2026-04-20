use std::fs::{read_to_string, File};
use std::io::{self, Write};

fn check_palindrome3(x: i32) -> String {
    if x < 100 || x > 999 {
        return "-".to_string();
    }
    let a = x / 100;
    let c = x % 10;
    if a == c {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

fn main() {
    let n: usize;
    let nums: Vec<i32>;

    if let Ok(content) = read_to_string("input.txt") {
        let data: Vec<i32> = content
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        n = data[0] as usize;
        nums = data[1..].to_vec();
    } else {
        let mut input = String::new();

        print!("Введите количество чисел N: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        n = input.trim().parse::<usize>().unwrap();

        input.clear();
        print!("Введите числа: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        nums = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    let mut results: Vec<String> = Vec::new();
    for i in 0..n {
        results.push(check_palindrome3(nums[i]));
    }

    let output = results.join(" ");
    println!("{}", output);

    let mut file = File::create("output.txt").unwrap();
    writeln!(file, "{}", output).unwrap();
}
