use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {

    let snacky_elves:Vec<i32> = parse_input("./resources/day1/input");
    calculate_top_3_snackers(&snacky_elves);
}

fn calculate_top_3_snackers(snacky_elves: &Vec<i32>){
    let max_calories = calculate_max_calories(snacky_elves);
    let mut second_max_calories = 0;
    let mut third_max_calories = 0;

    for x in snacky_elves {
        if x >= &second_max_calories && x< &max_calories {
            second_max_calories = *x;
        }

        if x >= &third_max_calories && x< &second_max_calories {
            third_max_calories = *x;
        }
    }

    println!("second most calories {}", second_max_calories);
    println!("third most calories {}", third_max_calories);
    println!("top 3 calories in total {}", max_calories + second_max_calories + third_max_calories);
}

fn calculate_max_calories(snacky_elves: &Vec<i32>) -> i32 {
    let mut max_calories: i32 = 0;

    for i in 0..snacky_elves.len() {
        if snacky_elves[i] >= max_calories {
            max_calories = snacky_elves[i];
        }
    }

    println!("max calories {}", max_calories);

    return max_calories;
}

fn parse_input(x: &str) -> Vec<i32>{
    let mut vector: Vec<i32> = Vec::new();
    vector.push(0);
    let mut last_index = 0;

    if let Ok(lines) = read_lines(x) {
        for line in lines {
            if let Ok(calories) = line {
                // println!("{}", calories);
                if calories.is_empty() {
                    vector.push(0);
                    last_index += 1;
                } else {
                    vector[last_index] += calories.parse::<i32>().unwrap();
                }
            }
        }
    }

    return vector;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


