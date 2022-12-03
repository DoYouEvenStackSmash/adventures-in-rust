fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(", hello");
    println!("{s}");
    var_interactions();
    ownership_passing();
    mutable_str();
}

fn var_interactions() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{:?}",s2);
}


fn ownership_passing() {
    let s1 = factory_string();  // s1 valid
    let s2 = String::from("hello"); // s2 valid
    let s3 = bounce_back(s2);   // s2 invalid
                                // s3 valid
    let s2 = factory_string();
    println!("s1: {}\ns2: {}\ns3: {}", s1, s2, s3);

}


// statement allocates for some string
// expression evaluates to some_string
// need a type for function
fn factory_string() -> String {
    let some_string = String::from("yours");
    some_string
}

// expression evaluates to a_str
fn bounce_back(a_str: String) -> String {
    a_str
}

// s is mutable in this context!
fn add_suffix_mut(mut s: String) -> String{
    s.push_str(" world");
    s
}
fn mutable_str() {
    // s is immutable in this context
    let s = String::from("hello");
    let s = add_suffix_mut(s);
    println!("{s}");
    /* does not work, fails ownership */ 
    // let r = String::from("HELLO");
    // let r = add_suffix_immut(r);
    // println!("{r}");
}

fn add_suffix_immut(mut s: String) -> String {
    s.push_str("WORLD");
    s
}