
fn main() {
  comments();



fn comments() {
  // stringified comment
  println!("|\t{} ", 3); // 3
  
  head(false);
  
  // positional comment
  println!("|\t{0}{0}", 1);  // 11
  
  head(false);
  
  // named arguments
  println!("|\ta,b,c: {a},{b},{c}",
    a = "A",
    b = "B",
    c = "C");
    // let top = String::new();
    // let bot = String::new();
    // let top : str = "|----------------------------------";
    // let bot : str = "|__________________________________";
  head(false);
  println!("|Different formatting by specifying after :");
  println!("|\tBase 10:          {}", 69420);
  println!("|\tBase 2:           {:b}", 69420);
  println!("|\tBase 8:           {:o}", 69420);
  println!("|\tBase 16(lhex):    {:x}", 69420);
  println!("|\tbase 16(uhex):    {:X}", 69420);
  
  head(false);
  // right justify
  println!("|\t{number:>5}", number=1);
  
  head(false);
  // named arguments with $
  println!("|\t{number:0>width$}", number=1, width=5);
  head(false);
}

// derive a debug implementation for the structure.
#[derive(Debug)]
struct person<'a> {
  name: &'a str,
  age: u8
}

fn debug_trait() {
  let name = "P";
  let age = 14;
  let p = person{name, age};
  println!("{:#?}", p);
}

fn head(t : bool) {
  if !t {
    println!("+----------------------------------");
  } else {
    println!("|__________________________________");
  }
  // println!("{t}");
}

use std::fmt;
// create a structure for display

struct Stx(i32);  // tuple struct containing i32

impl fmt::Display for Stx {
  // format must have this signature
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt:: Result {
    // write the first element to the supplied output stream f. returns result
    write!(f, "{}", self.0)
  }
}

