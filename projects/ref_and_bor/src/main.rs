fn main() {
    println!("Hello, world!");
    ref_as_param();
    borrow_test();
}


fn ref_as_param() {
    let s1 = String::from("Hello");
    let len = calculate_len(&s1);
    println!("length of {0} is {len}", s1);
}

fn calculate_len(s : &String) -> usize {
    s.len()
}

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