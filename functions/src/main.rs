fn main() {
    print(10);
    print(add(10, 20));

    // control flow
    let x = 10;
    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is less than 5");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // looping
    for i in 0..10 {
        print!("{} ", i);
    }
    println!();

    let mut a = [1, 2, 3, 4, 5];
    for element in a {
        if element < 5 {
            a[element] = 343;
            println!("element less than 5 {} ", a[element]);
        }
        println!("{} ", element);
    }
    
    for element in a {
        print!("{} ", element);
    }
}

fn print(x: i32) {
    println!("{}", x);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
