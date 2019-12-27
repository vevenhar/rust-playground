use std::env;

/*
 * Calculate greatest common divisor of two integers
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args); // prints all the arguments, including the program name
    if args.len() != 3 {
        panic!("must pass two integers");
    }
    let x: u32 = args[1].trim().parse().unwrap();
    let y: u32 = args[2].trim().parse().unwrap();
    println!("x is {}", x);
    println!("y is {}", y);
    println!("gcd is {}", gcd(x, y));
}

fn gcd(x: u32, y: u32) -> u32 {
    let p: u32;
    let q: u32;
    let mut result: u32 = 1;

    if x > y {
        p = x;
        q = y;
    } else {
        p = y;
        q = x;
    }

    for i in 1..p {
        if p % i == 0 && q % i == 0 {
            result = i;
        }
    }

    return result;
}
