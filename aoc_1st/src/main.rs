use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("combinations.txt")?;
    let reader = BufReader::new(file);

    let mut current: i32 = 0;
    let mut zeroes_p1: i32 = 0;
    let mut zeroes_p2: i32 = 0;
    let mut result: i32 = 50;

    for line in reader.lines() {
        current = result;
        let line = line?;
        let mut numer_val: i32 = line[1..].trim().parse().unwrap();
        zeroes_p2 += numer_val / 100;
        numer_val = numer_val % 100;
        if line.starts_with('L') {
            result -= numer_val;
            if result < 0 {
                result = 100 - result.abs();
                if current != 0 {
                    zeroes_p2 += 1;
                }
            }
        } else if line.starts_with('R') {
            result += numer_val;
            if result >= 100 {
                result -= 100;
                if result != 0 {
                    zeroes_p2 += 1;
                }
            }
        }

        if result == 0 {
            zeroes_p1 += 1;
        }
    }
    let passw_sum: i32 = zeroes_p1 + zeroes_p2;
    println!("{zeroes_p1} + {zeroes_p2} = {passw_sum}");
    Ok(())
}
