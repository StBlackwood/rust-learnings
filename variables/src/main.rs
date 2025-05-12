fn main() {
    println!("### Data Types ###");
    datatypes();
    println!();
    
    println!("### Mutability ###");
    mutability();
    println!();
}

fn datatypes() {
    let x = 33u8;
    println!("x is: {}", x);
    
    
    let y = 3435.0343f32;
    println!("y is: {}", y);

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient is: {}", quotient);
    println!("truncated is: {}", truncated);


    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    
    println!("heart_eyed_cat is: {}", heart_eyed_cat);
    println!("z is: {}", z);
    println!("c is: {}", c);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, _z) = tup;
    println!("x is: {}", x);
    println!("y is: {}", y);
    println!("z is: {}", tup.2);
    
    // arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("first is: {}", first);

    let zeroes = [0; 10];
    println!("zeroes is: {}", zeroes.len());
}

fn mutability() {
    let _x: i32 = 5; // immutable
    let mut _y = 5; // mutable

    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // shadowing
    let mut x = 5;
    x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "     ";
    // let mut spaces = "     ";
    // spaces = spaces.len(); // error: cannot assign to different type if made immutable
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
}
