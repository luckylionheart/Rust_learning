fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const TIME_OF_DAY:u32 = 60 * 60 * 24;

    println!("seconds per day: {}", TIME_OF_DAY);
}
