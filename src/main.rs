use std::fs::{self, remove_file};
use std::{io, result, task};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

use crate::math1::division;

macro_rules! my_print {
    ($msg:expr) => {
        println!("{}", $msg);    
    };
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }

    pub fn minus(a: i32, b: i32) -> i32 {
        return a - b;
    }
}

mod math1;

fn main() {
    // comment

    /* 
        Multi-line
        comment
    */ 
    println!("Hello, world!");

    // Integer: i8; i16; i32; i64; i128; u8; u16; u32; u64; u128
    println!("\n// Integer: i8; i16; i32; i64; i128; u8; u16; u32; u64; u128");
    let mut num: u8 = 50;
    num += 10;
    println!("Result: {}", num);

    let num = "hello";
    println!("Result: {}", num);

    // Float
    let num: f32 = 5.634;
    println!("Result: {}", num);

    // Boolean
    let num: bool = true;
    println!("Result: {}", num);

    // Char
    let sym: char = '%';
    println!("Result: {}", sym);

    
    // lesson 3 (Constants, tuples, and arrays)
    println!("\n\n\n// lesson 3 (Constants, tuples, and arrays)");
    // Constants
    println!("\n// Constants");
    const USER_MAX_SCORE: u32 = 1_000_000;
    println!("Info: {}", USER_MAX_SCORE);

    // Tuple
    println!("\n// Tuple");
    let user_alex: (i8, bool, f32, char) = (42, true, 1.86, 'R');
    println!("Alex data: ({}, {}, {}, {})", user_alex.0, user_alex.1, user_alex.2, user_alex.3);

    let mut user_gosha: (i8, bool, f32, char) = (42, true, 1.86, 'R');
    user_gosha.0 = 34;
    user_gosha.1 = false;
    user_gosha.3 = 'J';
    println!("Gosha data: ({}, {}, {}, {})", user_gosha.0, user_gosha.1, user_gosha.2, user_gosha.3);

    // Array
    println!("\n// Array");
    let nums: [i8; 5] = [1, 5, 6, 4, 4];
    println!("Nums: [{}, {}, {}, {}, {}]", nums[0], nums[1], nums[2], nums[3], nums[4]);

    let mut nums_mut: [i8; 5] = [1, 5, 6, 4, 4];
    nums_mut[1] = 2;
    nums_mut[2] = 3;
    nums_mut[3] = 4;
    nums_mut[4] = 5;
    println!("Nums mut: [{}, {}, {}, {}, {}]", nums_mut[0], nums_mut[1], nums_mut[2], nums_mut[3], nums_mut[4]);

    
    //// lesson 4 (Memory Management and Ownership)
    println!("\n\n\n// lesson 4 (Memory Management and Ownership)");
    //// User input
    println!("\n// User input");
    let mut user_data = String::new();
    println!("\n// Input something");
    io::stdin().read_line(&mut user_data).expect("Fail to read information");
    println!("Result: {}", user_data);

    // let mut num1 = String::new();
    // let mut num2 = String::new();
    // println!("Enter num1: ");
    // io::stdin().read_line(&mut num1).expect("Fail to read information");

    // println!("Enter num2: ");
    // io::stdin().read_line(&mut num2).expect("Fail to read information");

    // let num1: i16 = num1.trim().parse().expect("Pleese enter a valid number");
    // let num2: u8 = num2.trim().parse().expect("Pleese enter a valid number");
    // println!("Result 1: {}, result 2: {}", num1, num2);

    // let mut res: i16 = num1 + num2 as i16;
    // println!("Result: {}", res);

    // res += num1 - num2 as i16;
    // println!("Result: {}", res);


    //// Ownership
    // println!("\n// Ownership");
    // let s1 = String::from("Hello");
    // let s2 = s1;
    // println!("Ownership: {}", s1); // Error

    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);

    let mut s = String::from("Hello");
    change(&mut s);
    // println!(s); Error
    println!("{}", s);

    let mut s = String::from("Hello");
    let r1 = &s; // Не изменяемое заимсвтование
    let r2 = &s; // Ещё одно заимстовование
    let r3 = &mut s; // Ошибка, нельзя создвать изменяемое заимстваование, пока существует хот одна изменяемая ссылка 
    // println!("{}, {}, {}", r1, r2, r3);
/*     
    error: 3 positional arguments in format string, but there is 1 argument
    --> src/main.rs:104:15
        |
    104 |     println!("{}, {}, {}", r3);
        |               ^^  ^^  ^^   --

    error: could not compile `learning_rust` (bin "learning_rust" test) due to 1 previous error
*/ 

    println!("{}", r3);


    // lesson 5 (Conditional constructs: if-else, match)
    println!("\n\n\n// lesson 5 (Conditional constructs: if-else, match)");
    // if-else
    println!("\n// if-else");
    let number: u8 = 10;
    let is_has_car: bool = true;
    
    if number > 5 && is_has_car {
        println!("Number is bigger than 5");
    } else if number > 10 || !is_has_car {
        println!("Number is bigger than 10");
    } else {
        println!("Else");
    }

    // Ternary operator
    println!("\n// Ternary operator");
    let condition: bool = true;
    let number: u8 = if condition {5} else {10};
    println!("Ternary operator res: {}", number);

    // Operator match
    println!("\n// Operator match");
    let number: u8 = 3;
    
    match number {
        1 => println!("Result 1"),
        2 => println!("Result 2"),
        3 => println!("Result 3"),
        4 => println!("Result 4"),
        5 => println!("Result 5"),
        _ => println!("Else")
    }

    // lesson 6 (Loops and operators)
    println!("\n\n\n// lesson 6 (Loops and operators)");
    // Loops Rust
    println!("\n// Loops Rust");
    // For
    println!("\n// For");
    for i in 1..4 {
        println!("i (for i in 1..4): {}", i);
    }
    println!("");
    for i in 1..=4 {
        println!("i (for i in 1..=4): {}", i);
    }
    println!("");
    for i in (1..=4).rev() {
        println!("i (for i in (1..=4).rev()): {}", i);
    }
    println!("");
    for i in (1..=10).rev().step_by(2) {
        println!("i (for i in (1..=10).rev().step_by(2)): {}", i);
    }

    // While
    println!("\n// While");
    let mut i: u8 = 8;
    while i > 0 {
        println!("i (while i > 0): {}", i);
        i -= 1;
    }
    println!("");
    for i in 1..21 {
        if i % 2 == 0 {
            continue;
        }
        println!("for !(i % 2 == 0) in 1..21: {}", i);
        if i > 15 {
            break;
        }
    }

    // Loop
    println!("\n// oop");
    let mut count: u8 = 0;
    loop {
        println!("Count (loop): {}", count);
        count += 1;
        if count >= 128 {
            break;
        }   
    }

    // Arrays and Loops
    println!("\n// Arrays and Loops");
    let array = [10, 20, 30, 40, 50];
    for el in array {
        println!("el (for el in array): {}", el);
    }
    for el in [1, 2, 3, 4, 5] {
        println!("el (for el in [1, 2, 3, 4, 5]): {}", el);
    }

    // lesson 7 (Functions and modules)
    println!("\n\n\n// lesson 7 (Functions and modules)");
    // Functions
    println!("\n// Functions");
    test();
    test();
    add(6, 4);
    add(43, 54);
    print!("\n\n");
    greet_user("user");
    greet_user("Test user");

    // Change paramter in function
    println!("\n// Change paramter in function");
    let mut user = String::from("User change test");
    let mut user1 = String::from("Test change user");
    println!("user: {}\nuser1: {}", user, user1);
    change_str(&mut user, &mut user1);
    println!("user: {}\nuser1: {}", user, user1);

    // Return
    println!("\n// Return");
    let resuslt = add(4, 5);
    println!("Return: {}", resuslt);
    println!("mult(5, 8): {}", mult(&(5, 8)));

    // Macros rules
    println!("\n// Macros rules");
    my_print!("Hallo, Rust");

    // Modules
    println!("\n// Modules");
    let sum = math::add(4, 9);
    let ras = math::minus(7, 4);
    println!("sum: {}\nras: {}", sum, ras);
    let mult = math1::mult(4, 9);
    let division = math1::division(7, 4);
    println!("mult: {}\ndivision: {}", mult, division);

    // lesson 8 (Vectors, strings and collections)
    println!("\n\n\n// lesson 8 (Vectors, strings and collections)");
    //Vectors
    println!("\n// Vectors");
    let mut vector: Vec<i32> = Vec::new();
    vector.push(10);
    vector.push(20);
    vector.push(30);
    vector.push(40);
    println!("Vector: {:?}", vector);
    vector[0] = 256;
    println!("Vector[0]: {:?}", vector[0]);
    println!("Vector: {:?}", vector);

    let mut vector2 = vec![1, 5, 3, 6, 7];
    vector2.push(56);
    println!("Vector2: {:?}", vector2);

    let index: usize = 10;
    match vector2.get(index) {
        Some(value) => println!("Vector2 el: {}", value),
        None => println!("Error no element"),
    }

    // Vector (pop), for value in vector
    println!("\n// Vector (pop), for value in vector");
    let mut vector3 = vec![100, 200, 300];
    vector3.pop();
    vector3.push(1024);
    vector3.remove(1);
    for value in &vector3 {
        println!("vector3 el: {}", value);
    }

    // Strings
    println!("\n// Strings");
    let str1 = String::new();
    let str2 = String::from("Hello, Rust");
    println!("str1: {}\nstr2: {}", str1, str2);

    let str3 = str1 + &str2;
    let mut word = String::new();
    word.push_str("Hello");
    word.push(' ');
    word.push_str("Rust");
    word.push('\n');
    word.push_str(&str3);
    println!("\nString test push and push_str \n{}", word);

    // HashMap
    println!("\n// HashMap");
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 5);
    scores.insert("Grey", 30);
    println!("{:?}", scores);

    scores.insert("Red", 15);
    println!("{:?}", scores);
    println!("Hash['Red']: {:?}", scores.get("Red"));
    println!("Hash['Red']: {:?}", scores.get("Red").unwrap());
    println!("Hash['Red']: {}", scores.get("Red").unwrap());

    scores.remove("Grey");
    println!("{:?}", scores);

    // lesson 9 (Errors and processing)
    println!("\n\n\n// lesson 9 (Errors and processing)");
    // Resul
    let result = divide(4, 4).unwrap();
    println!("Resul: {}\n", result);
    let result = divide(4, 2);
    println!("Resul1: {:?}\n", result);
    // let result = divide(4, 0).expect("Error in division");
    // println!("Resul2: {}\n", result);
    // let result = divide(4, 0);
    // println!("Resul3: {:?}\n", result);

    let element = find_element(vec![1, 2, 3, 4, 5], 3);
    println!("{:?}", element);
    let element = find_element(vec![1, 2, 3, 4, 5], 32);
    println!("{:?}", element);
    // let element = find_element(vec![1, 2, 3, 4, 5], 32).unwrap();
    // println!("{:?}", element);
    
    let result = divide(7, 0);
    match result {
        Ok(value) => println!("{}", value),
        Err(e) => println!("Error in function: {}", e),
    }
    
    // Files
    println!("\n// Files");
    let file_name = "output.txt";
    match write_to_file(file_name, "Hello, Rust!") {
        Ok(()) => println!("Data is written"),
        Err(e) => println!("Error: {}", e),
    } 
    let file_name = "output.txt";
    match read_file(file_name) {
        Ok(content) => println!("Content: {}", content),
        Err(e) => println!("Error: {}", e),
    }

     // lesson 10 (Introduction to Multithreading)
    println!("\n\n\n// lesson 10 (Introduction to Multithreading)");
    // Threads
    println!("\n// Threads");
    let duration = Duration::from_micros(1000);
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_micros(1000));
        };
    });
    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(duration);
    }
    handle.join().unwrap();

    // Passing variables to threads
    println!("\n// Passing variables to threads");
    let data = vec![1, 6, 3, 7, 90];
    let handle = thread::spawn(move || {
        for i in data {
            println!("Thread: {}", i);
            thread::sleep(duration);
        };
    });
    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(duration);
    }
    handle.join().unwrap();

    // Passing variables to threads with only one vector
    println!("\n// Passing variables to threads with only one vector");
    let data = vec![1, 6, 3, 7, 90];
    let handle = thread::spawn({
        let data_clone = data.clone();
        move || {
            for i in data_clone {
                println!("Thread: {}", i);
                thread::sleep(duration);
            };
        }
    });
    for i in data {
        println!("Main thread: {}", i);
        thread::sleep(duration);
    }
    handle.join().unwrap();

    // Async & Await
    println!("\n// Threads");
    async_and_await();
}

fn change_str(user: &mut String, user1: &mut String) {
    // let buf = user;
    // *user = user1;
    // *user1 = buf;
    *user = String::from("user1");
    *user1 = String::from("user");
    println!("changed");
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(", World");
}

fn test() {
    println!("test (Hallo)")
}

fn add(a: i32, b: i32) -> i32 {
    let res = a + b;
    print!("add (a + b = {})", res);
    println!("test print ln and print");
    return res;
}

fn greet_user(name: &str) {
    println!("Name: {}", name);
}

fn mult(data: &(i32, i32)) -> i32 {
    return data.0 * data.1;
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Division by zero"));
    } else {
        return Ok(a / b);
    }
}

fn find_element(vec: Vec<i32>, value: i32) -> Option<usize> {
    return vec.iter().position(|&x| x == value);
}

fn write_to_file(file_path: &str, content: &str) -> Result<(), io::Error> {
    return fs::write(file_path, content);
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
    return fs::read_to_string(file_path);
}

#[tokio::main]
async fn async_and_await() {
    let task1 = simulate_download("File 1", 2).await;
    let task2 = simulate_download("File 2", 3).await;
    println!("{} and {} are download!", task1, task2);
    
    let (task1, task2) = tokio::join!(
        simulate_download("File 1", 3),
        simulate_download("File 2", 2),
    );
    println!("{} and {} are download!", task1, task2);
}

async fn simulate_download(file_name: &str, seconds: u64) -> String {
    println!("Starting to download {}", file_name);
    sleep(Duration::from_secs(seconds)).await;
    println!("Finished to download {}", file_name);
    return file_name.to_string();
}
