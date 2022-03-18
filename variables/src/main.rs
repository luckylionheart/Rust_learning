use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const TIME_OF_DAY:u32 = 60 * 60 * 24;

    println!("seconds per day: {}", TIME_OF_DAY);

    let heart_eyed_cat = '😻';

    println!("heart eyed cat {}", heart_eyed_cat);

    let a = [3; 5];
    println!(" {}", a[4]);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    another_function();
    // error code, different type of result

    // let mut spaces = "    ";
    // spaces = spaces.len();
}

fn another_function() {
    let y = {
        let x = 3;
        x + 1
    };
    let z = five();
    println!("It's call of another function and show the y and z: {} {}", y, z);

    //check the condition
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("make the condition and check the if case {}", number);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn five() -> i32 {
    5 //in that case, you should not use the ;(return the value)
}
