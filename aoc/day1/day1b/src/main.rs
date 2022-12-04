use std::fs;
const DELIM:char = '\n';
fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = read_str(v);
    v.sort();
    println!("{v:#?}");
    let vsl = &v[v.len()-3..];
    let total : i32 = vsl.iter().sum();
    println!("sum: {}",total);
    // println!("{vsl:#?}");
}

fn read_str(mut v : Vec<i32>) -> Vec<i32> {
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
        v.push(curr_sum);

        elf_count += 1;
        curr_sum = 0;
    }
    
    v.push(curr_sum);
    
    v
}