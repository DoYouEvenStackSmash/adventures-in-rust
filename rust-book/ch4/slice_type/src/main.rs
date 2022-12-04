fn main() {
    println!("Hello, world!");
    iterate();
    slice_syntax();
    let s = String::from("abcdefghijkl  mnopqrstuvwxyz");
    let r: &str = first_word(&s);
    println!("{r}");
    let str_new  = String::from(&s[r.len()..].to_string());
    println!("{str_new}");
    let str_new : &str = find_next_char(&str_new);
    let rval = String::from(str_new.to_string());
    let r2: &str = first_word(&rval);
    println!("second word {r2}");
}
fn find_next_char(s : &String) -> &str {
    let bs = s.as_bytes();
    for (i, &item) in bs.iter().enumerate() {
        if item != b' ' {
            return &s[i..];
        }
    }
    &s[s.len() - 1..]
}
// fn second_word( s: &String) -> &str {
//     let bs = s.as_bytes();
//     for (i , &item) in bs.iter().enumerate() {
//         if 
//     }
// }

fn first_word( s: &String) -> &str {
    let bs = s.as_bytes();
    for (i, &item) in bs.iter().enumerate() {
        if item == b' ' {
            return &s[..i + 1];
        }
    }

    &s[..]
}
fn slice_syntax() {
    let s = String::from("abcdefghijkl  mnopqrstuvwxyz");
    // from beginning of string
    let s1 = &s[0..2];
    let s2 = &s[..2];
    println!("{s1}\n{s2}");

    // to end of string
    let s1 = &s[3..s.len()];
    let s2 = &s[3..];
    println!("{s1}\n{s2}");

    // entire string
    let s1 = &s[0..s.len()];
    let s2 = &s[..];
    println!("{s1}\n{s2}");

}

fn iterate() {
    let s = String::from("abcdefghijklmnopqrstuvwxyz");
    let bs = s.as_bytes();
    for (idx, &c) in bs.iter().enumerate() {
        println!("{idx}:{0}",c);
    }
}
/*
= note: the only appropriate formatting traits are:
- ``, which uses the `Display` trait
- `?`, which uses the `Debug` trait
- `e`, which uses the `LowerExp` trait
- `E`, which uses the `UpperExp` trait
- `o`, which uses the `Octal` trait
- `p`, which uses the `Pointer` trait
- `b`, which uses the `Binary` trait
- `x`, which uses the `LowerHex` trait
- `X`, which uses the `UpperHex` trait
*/