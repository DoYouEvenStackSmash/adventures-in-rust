fn main() {
    println!("Hello, world!");
    a_fxn();
    a_fxn1(8);
    let x: [i32; 2] = [1,2];
    a_multi_fxn(x);
    dec_type(x[1],'h');

    // functions that return values
    let k = expr();
    println!("{k}\n");
    let k = plus_one(k);
    println!("{k}\n");
    wack();
}

// snake case

fn a_fxn() {
    println!("afxn");
}

fn a_fxn1(x: i32) {
    println!("the value of x is {x}");    
}

fn a_multi_fxn(r : [i32;2]) {
    println!("{r:?}");
}

fn dec_type(j:i32, c: char ) {
    println!("{j:?}{c}");
}


fn wack() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("{y}");
}


/*
    functions that return values
*/

fn expr() -> i32{
    let y = 6;
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}