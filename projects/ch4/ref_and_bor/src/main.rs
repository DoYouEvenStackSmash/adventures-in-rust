fn main() {
    //incr_head();
    toy_mut();
    /*
    println!("Hello, world!");
    ref_as_param();
    borrow_test();
    println!("\n");
    mut_ref_test();*/
}


fn ref_as_param() {
    let s1 = String::from("Hello");
    let len = calculate_len(&s1);
    println!("length of {0} is {len}", s1);
}

fn calculate_len(s : &String) -> usize {
    s.len()
}

// some confusing logic to exercise ownership, return types and references
fn borrow_test() {
    let s = String::from("AAAA");   // immutable
    println!("\npassing");
    println!("{s}");
    let k = pass_through(&s);
    println!("borrow_test: from {} to val {k}\n", s);
    let s2 = pass_through2(&s);
    println!("{} of len {1}", s, s2);
}

fn pass_through(s : &String) -> usize{
    let slen = s.len();
    println!("pass_through: {} is of len {slen}",s);
    let kstr = pass_through2(s);
    kstr.len()
}

fn pass_through2(s : &String) -> String {
    println!("pass_through2: {}",s);
    let k = s.len();
    let retstr = String::from(k.to_string());
    retstr
}


fn toy_ref() {
    let s = String::from("hi");
    let s_ref:&String = &s;
    println!("{s_ref}");
}

//  uses mutable reference and ownership to add to an immutable string
fn mut_ref_test() {
    toy_ref();
    println!("\nMutable Reference operations");
    let k1 = String::from("X");

    print_section(&k1);
    let k1 = bounce_back(k1);
    print_section(&k1);

    let k1 = bounce_back_2(k1);
    print_section(&k1);
    println!("\n");
    
}

fn print_section(s : &String) {
    print!("{s}\t");
}
// takes ownership of string, changes, returns
fn bounce_back(mut s : String) -> String {
    s.push_str("BB");
    s
}

fn bounce_back_2(s: String) -> String {
    let mut s = bounce_back(s);
    change(&mut s);
    s
}

fn change(s : &mut String) {
    s.push_str("CC");
}

fn incr(n: &mut i32) {
    *n += 1;
}

fn incr_head() {
    let mut n = 1;
    incr(&mut n);
    println!("{n}");
}
fn toy_mut() {
    let mut s = String::from("hello");
    let s2 = &s;
    let s3 = &mut s;
    s3.push_str(" world");
    let s2 = &s;
    println!("{s2}");
}