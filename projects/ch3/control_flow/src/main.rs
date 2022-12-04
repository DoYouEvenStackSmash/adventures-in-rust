fn main() {
    
    println!("Hello, world!");
    fxn_if();
    ternary_fxn();
    test();
    some_loops();

}


fn ternary_fxn() {
    let cond = true;
    let k = if cond {5} else {6};   // ternary operator
    println!("{k}");

    // let l = if cond { 5 } else { "six" };   // mismatched types
    // println!("{l}");
}

fn fxn_if() {
    let num = 7;
    if num < 5 {
        println!("cond = true");
    } else {
        println!("cond = false");
    }
    if num != 0 {   // must provide boolean to if expression
        println!("number is {num}");
    }
}

fn test() {
    let y = if true {};
    println!("{y:?}");
}

fn some_loops() {
    for_loops();
    loop_labels();
    while_loop();
    while_loop_arr();
}

fn for_loops() {
    let mut x = 0;
    // escape a value out of a for loop by putting the return 
    // beyond the end condition
    let result = loop {
        println!("again!");
        x = x + 1;
        if x < 10 {
            continue;
        }
        break x + 100;
    };
    println!("result {result}");

}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut rem = 10;

        loop {
            println!("rem = {rem}");
            if rem == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            rem -= 1;
        }
        count +=1;
    }
    println!("end count = {count}")
}

fn while_loop() {
    let mut num = 5;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    println!("{num}");
}

fn while_loop_arr(){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("{}",a[index]);
        index += 1;
    }
    for_loop_arr();
}

fn for_loop_arr() {
    let a = [10, 20, 30, 40, 50];
    println!("\n");
    for num in a {
        println!("{num}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
}