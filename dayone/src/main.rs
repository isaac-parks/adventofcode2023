use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn get_input() -> Result<BufReader<File>, io::Error> { 
    let file = File::open("./part1input")?;
    let reader = BufReader::new(file);

    Ok(reader)
}


fn main() {
    let reader:  Result<BufReader<File>, io::Error> = get_input();
    let s_nums: Vec<&str> = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let num_chars = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let s_num_map: HashMap<&str, char> = s_nums.iter().cloned().zip(num_chars.into_iter()).collect();

    let mut sum: u128 = 0;

    for line in reader.unwrap().lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        let mut l = 0;
        let mut r = line.chars().count() - 1;

        let mut word: String = String::new();
        let mut s_num = String::new();

        let mut pushed = false;

        while l <= r {
            if chars[l].is_numeric() {
                s_num.push(chars[l]);
                pushed = true;
            } else {
                word.push(chars[l]);
                for word_num in s_nums.clone() {
                    if word.contains(&word_num) {
                        s_num.push(s_num_map.get(&word_num).unwrap().clone());
                        pushed = true;
                        break;
                    }
                }
            }

            if pushed {
                break;
            }

            l += 1;
        }

        word = String::new();
        pushed = false;

        while r >= l {
            if chars[r].is_numeric() {
                s_num.push(chars[r]);
                pushed = true;
            }
            else {
                word.insert(0, chars[r]);
                for word_num in s_nums.clone() {
                    if word.contains(&word_num) {
                        s_num.push(s_num_map.get(&word_num).unwrap().clone());
                        pushed = true;
                        break;
                    }
                }
            }

            if pushed {
                break;
            }

            r -= 1;
        }

        let num: u128 = s_num.parse().unwrap();
        sum += num
    }

    println!("{}", sum);
}