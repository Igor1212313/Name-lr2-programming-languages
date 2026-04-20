use std::fs::{read_to_string, File};
use std::io::{self, Write};

fn min_refuels(n: usize, v: i32, r: &Vec<i32>) -> i32 {
    for &dist in r {
        if dist > v {
            return 0;
        }
    }

    let mut fuel = v;
    let mut refuels = 0;

    for i in 0..(n - 1) {
        if fuel < r[i] {
            return 0;
        }

        fuel -= r[i];

        if i < n - 2 && fuel < r[i + 1] {
            refuels += 1;
            fuel = v;
        }
    }

    refuels
}

fn main() {
    let mut n: usize = 0;
    let mut v: i32 = 0;
    let mut r: Vec<i32> = Vec::new();

    if let Ok(content) = read_to_string("input.txt") {
        let nums: Vec<i32> = content
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        n = nums[0] as usize;
        v = nums[1];
        r = nums[2..].to_vec();
    } else {
        let mut input = String::new();

        print!("Введите количество планет N: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        n = input.trim().parse::<usize>().unwrap();

        input.clear();
        print!("Введите емкость бака V: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        v = input.trim().parse::<i32>().unwrap();

        input.clear();
        print!("Введите расстояния между соседними планетами: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        r = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
    }

    let result = min_refuels(n, v, &r);

    println!("Результат: {}", result);

    let mut file = File::create("output.txt").unwrap();
    writeln!(file, "{}", result).unwrap();
}
