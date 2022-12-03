use std::io;
const KNOWN_CONST: f64 = 4.0;
fn main() {
    //let mut x = 5;
    //println!("the value of x is {x}");
    // let x = 6;
    // println!("the value of x is {x}");

    let z = 5;
    println!("the value of x is {z}");
    let z = z + 1;
    println!("the value of x is {z}");

    {
            let z = z * 2;
            println!("tne value of x is {z}");
    }
    println!("the value of x is {z}");

    let spaces = "   ";
    let spaces = spaces.len(); // shadowing

    // let mut space = "   ";
    // space = space.len();  // string type cannot be modified?
    println!("\n");
    
    // ieee standard
    let x = 2.0;    // f64 default
    let y: f32 = 3.0;
    println!("{x}");    // f32
    
    // arithmetic: each evaluates to a single value, then binds to a variable
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / KNOWN_CONST;

    let floored = 2 / 3;    // results in a zero

    let remainder = 43 % 5;

    println!("{quotient}");
    // prefix unused variables with _

    let t = true;
    let f: bool = false;
    println!("{t}");

    let c = 'z';
    let z: char  = 'ℤ';
    // let h: char = 'U+005a';
    println!("{z}");

    //tuples

    let tup: (i32, f64, u8) = (500, 4.0, 1);
    
    //  destructuring
    let (x, y, z) = tup;
    println!("val of y = {y}");
    println!("{tup:?}");
    // indexing a tuple is done with a .
    let (a, b, c) = (tup.1, tup.0, tup.2);
    println!("norm x: {x}\nswap x: {a}");
    // println!("tup(0): {tup.0}"); // does not work

    // arrays have length
    // : does pretty print?
    let a = [1, 2, 3, 4, 5];

    println!("{a:#?}");
    
    let mon = ['a', 'b', 'c'];
    println!("{mon:?}");
    let mon_1 = mon[1];

    println!("mon_1: {mon_1}");
    let mut ints: [i32; 3] = [1; 3];
    println!("{ints:?}");   //[1, 1, 1]

    let second = ints; 
    ints[0] = 3;    
    println!("{second:?}"); // [1, 1, 1]
    println!("{ints:?}");   // [3, 1, 1]

    // out of bounds

    let a = [1; 3];
    let b = a.len();
    println!("{b}");
    
    /*  does not compile
    let message = "foobarbaz";
    let str = [message, 100];
    */
    let t = ([1; 2], [3;4]);
    let (a, _) = t;
    println!("{}", a[0] + t.1[0]);
    // println!("{}",t:?);

    //continue;
    // let mut index = String::new();
    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("failed to read line!");
    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Not a number");
    // let element = a[index];
    // println!("{a:?}\nthe value at {index} is {element}");
}
