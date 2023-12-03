use std::collections::HashMap;
use crate::problem::Problem;

pub struct Day1 {
    pub input: Vec<String>,
}

impl Day1 {

    fn join_digits(line: &str) -> u32 {
        let (mut i, mut j) = (0, line.len() - 1);
        while i <= j {
            // extract first and last chars based on i and j
            let first = line.chars().nth(i).unwrap();
            let last = line.chars().nth(j).unwrap();

            // determine if they are numerical digits within radix 10
            let first_is_digit = first.is_digit(10);
            let last_is_digit = last.is_digit(10);

            // combine as string, then parse as i32
            if first_is_digit && last_is_digit {
                let mut result = first.to_string().to_owned();
                result.push_str(&last.to_string());

                let num = result.parse().unwrap();
                // println!(", result: {num}");
                return num;
            }

            // only move pointers if we have not arrived at a digit yet
            i += !first_is_digit as usize;
            j -= !last_is_digit as usize;
        }
        panic!("Got invalid input: {}", line);
    }

    fn replace_digits(digits_map: &HashMap<&str, u32>, line: &str) -> u32 {
        // replace char digits or spelled digits from start, going forward
        let first = (0..line.len())
            .find_map(|i| digits_map.keys()
                .find(|key| line[i..]
                    .starts_with(*key)))
            .unwrap();

        // replace char digits or spelled digits from end, going backward
        let last = (0..line.len())
            .find_map(|i| {
                digits_map.keys()
                    .find(|key| line[..(line.len() - i)]
                        .ends_with(*key))
            })
            .unwrap();

        digits_map[first] * 10 + digits_map[last]
    }
}

impl Problem for Day1 {
    fn solve_part_one(&self) -> String {
        self.input.to_owned().into_iter()
            .map(|n| Day1::join_digits(&n))
            .sum::<u32>()
            .to_string()
    }

    fn solve_part_two(&self) -> String {
        let digits: HashMap<&str, u32> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);

        self.input.to_owned().into_iter()
            .map(|line| Day1::replace_digits(&digits, &line))
            .sum::<u32>()
            .to_string()
    }
}