fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`

    let s1 = gives_ownership();
    println!("{s1}");

    let mut s2 = String::from("hello");
    s2 = takes_and_gives_back(s2);

    println!("{s2}");

    let (len, s3) = calculate_length(s2);
    println!("length: {len}, string: {s3}");

    let mut s = String::from("hello");

    let x = &mut s;
    change(x);
    println!("{s}");

    // cannot borrow multiple times at once
    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // let mut s = String::from("hello");
    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);
    // println!("{}, {}, and {}", r1, r2, r3);

    // this is okay, coz, references are not being used after a new mutable reference is created
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    // s = String::from("world"); // this will throw error, since s is already borrowed
    let r2 = &s; // no problem
    println!("{}", s.len());
    println!("{r1} and {r2}");
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
fn gives_ownership() -> String {
    let s = String::from("some string");
    s
}

fn takes_and_gives_back(s: String) -> String {
    println!("{s}");
    s
}

fn calculate_length(s: String) -> (usize, String) {
    (s.len(), s)
}
