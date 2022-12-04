use std::io;
use std::fs;
const DELIM:char = '\n';

fn main() {
    let ans: (i32, i32) = read_str();
    println!("{} has {} cal", ans.0 ,ans.1);
}

fn read_str() -> (i32, i32) {
    let filename = "/home/aroot/repos/adventures-in-rust/aoc/input.txt";
    
    let mut curr_sum: i32 = 0;
    let mut max_sum: i32 = 0;

    let mut elf_id: i32 = 0;
    let mut elf_count: i32 = 1;

    let input = fs::read_to_string(filename).expect("unable to read filename");
    // let input = test;

    for elem in input.split(DELIM) {    
        
        if elem.len() != 0 {
            let value:i32 = elem.parse().expect("could not unpack value");
            curr_sum += value;
            continue;
        }

        if max_sum < curr_sum {
            elf_id = elf_count;
            max_sum = curr_sum;
        }

        elf_count += 1;
        curr_sum = 0;
    }
    
    if max_sum < curr_sum {
        elf_id = elf_count;
        max_sum = curr_sum;
    }
    
    (elf_id, max_sum)
}


