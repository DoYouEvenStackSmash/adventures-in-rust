fn main() {
    println!("Hello, world!");
    arithmetic();
}
fn struct_ops() {
    let u = User {
        email: String::from("foo@bar"),
        uname: String::from("foo"),
        active: true,
        sic: 1,
    };
    // debug print whole struct
    println!("{u:?}");
    // dot notation field access
    println!("{0}",u.email);
    // let x:&User = &u;
    
    let u = change_field(u);
    
    // println!("{0}",x.email);
}

#[derive(Debug)]
struct Pt {x:i32,y:i32}

fn arithmetic() {
    let mut a = Pt{x:1,y:2};
    a.x+=1;
    let b = Pt{y:1,..a};
    a.x+=1;

    let r1 = &mut a.x;
    let r2 = &mut a.y;
    *r2 = *r1;
    *r1 += 1;
    println!("{}:{}",a.x,a.y);
}

fn change_field(mut u: User) -> User {
    mut_str(&mut u.email);
    // u.email = String::from("baz@bar");
    u
}

fn mut_str(s: &mut String){// -> String {
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


#[derive(Debug)]
struct User {
    active: bool,
    uname: String,
    email: String,
    sic: u64,
}
