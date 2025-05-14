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
