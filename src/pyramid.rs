use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn max_path_pyramid<P>(path: P) -> usize
where P: AsRef<Path> {
    let file = File::open(path).unwrap();
    let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let height = lines.next().unwrap().parse::<usize>().unwrap();
    let mut numbers = vec![0; height*(height+1)/2];
    let mut max = 0;
    let mut current_index = 0;
    for k in 0..height {
        let mut current_row_index = 0;
        for number_str in lines.next().unwrap().split(' ') {
            let mut new_number = number_str.parse::<usize>().unwrap();
            if k > 0 {
                if current_row_index == 0 {
                    let predecessor_right = current_index - k;
                    new_number += numbers[predecessor_right];
                } else if current_row_index == k {
                    let predecessor_left = current_index - k - 1;
                    new_number += numbers[predecessor_left];
                }
                else {
                    let predecessor_left = current_index - k - 1;
                    let predecessor_right = current_index - k;
                    let max_predecessor_value = numbers[predecessor_left].max(numbers[predecessor_right]);
                    new_number += max_predecessor_value;
                }
                
                if k == height - 1 && new_number > max {
                    max = new_number;
                }
            }
            numbers[current_index] = new_number;
            current_index += 1;
            current_row_index += 1;
        }
    }
    max
}

pub fn write_max_path_pyramid<P>(input_path: P, output_path: P) -> std::io::Result<()>
where P: AsRef<Path> {
    let result = max_path_pyramid(input_path);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(result.to_string().as_bytes())?;
    Ok(())
}
