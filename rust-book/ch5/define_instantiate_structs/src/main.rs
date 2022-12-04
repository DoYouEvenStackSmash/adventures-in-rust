
#[derive(Debug)]
struct User {
    active: bool,
    uname: String,
    email: String,
    sic: u64,
}

#[derive(Debug)]
struct Pt {x:i32,y:i32}

fn main() {
    println!("Hello, world!");
    struct_ops();
    arithmetic();
}
fn struct_ops() {
    println!("\nstruct_ops");
    let u = User {
        email: String::from("foo@bar"),
        uname: String::from("foo"),
        active: true,
        sic: 1,
    };
    println!("\t+--premod---");
    // debug print whole struct
    println!("\t| debug:\t{u:?}");
    // dot notation field access
    println!("\t| dot_acc:\t{0}",u.email);
    let u = change_field(u);
    println!("\t+--postmod---");
    println!("\t| {0}",u.email);
    
}

fn change_field(mut u: User) -> User {
    mut_str(&mut u.email);
    u
}

fn mut_str(s: &mut String) {
    s.replace_range(0..1,"x");
}

fn build_user(email: String, uname: String) ->User {
    User {
        email: email,
        uname: uname,
        active: true,
        sic: 1,
    }
}
fn init_build_user(email: String, uname: String) ->User {
    User {
        email,
        uname,
        active:true,
        sic:1,
    }
}

fn arithmetic() {
    create_from_struct();
    modify_struct_fields();
}

fn create_from_struct() {
    println!("\ncreate_from_struct");
    let mut a = Pt{x:1,y:2};
    a.x+=1; // a.x +=1
    let b = Pt{y:1,..a};    // new b.y, a.x copied
    a.x+=1; //a.x +=1
    println!("a.x:{}\nb.x:{}",a.x,b.x);
}

fn modify_struct_fields() {
    println!("\nmodify_struct_fields");
    let mut a = Pt{x:1,y:2};
    println!("\t+--premod---");
    println!("\t| a.x: {}\n\t| a.y: {}",a.x,a.y);
    let r1 = &mut a.x;
    let r2 = &mut a.y;
    *r1 = *r2; // x == y
    *r1 += 1;   // x += 1
    println!("\t+--postmod---");
    println!("\t| a.x: {}\n\t| a.y: {}",a.x,a.y);
}



