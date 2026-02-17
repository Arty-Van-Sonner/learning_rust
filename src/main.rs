use std::io;

fn main() {
    // comment

    /* 
        Multi-line
        comment
    */ 
    println!("Hello, world!");

    // Integer: i8; i16; i32; i64; i128; u8; u16; u32; u64; u128
    println!("\nInteger: i8; i16; i32; i64; i128; u8; u16; u32; u64; u128");
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
    println!("\n\n\nlesson 3 (Constants, tuples, and arrays)");
    // Constants
    println!("\nConstants");
    const USER_MAX_SCORE: u32 = 1_000_000;
    println!("Info: {}", USER_MAX_SCORE);

    // Tuple
    println!("\nTuple");
    let user_alex: (i8, bool, f32, char) = (42, true, 1.86, 'R');
    println!("Alex data: ({}, {}, {}, {})", user_alex.0, user_alex.1, user_alex.2, user_alex.3);

    let mut user_gosha: (i8, bool, f32, char) = (42, true, 1.86, 'R');
    user_gosha.0 = 34;
    user_gosha.1 = false;
    user_gosha.3 = 'J';
    println!("Gosha data: ({}, {}, {}, {})", user_gosha.0, user_gosha.1, user_gosha.2, user_gosha.3);

    // Array
    println!("\nArray");
    let nums: [i8; 5] = [1, 5, 6, 4, 4];
    println!("Nums: [{}, {}, {}, {}, {}]", nums[0], nums[1], nums[2], nums[3], nums[4]);

    let mut nums_mut: [i8; 5] = [1, 5, 6, 4, 4];
    nums_mut[1] = 2;
    nums_mut[2] = 3;
    nums_mut[3] = 4;
    nums_mut[4] = 5;
    println!("Nums mut: [{}, {}, {}, {}, {}]", nums_mut[0], nums_mut[1], nums_mut[2], nums_mut[3], nums_mut[4]);

    
    //// lesson 4 (Memory Management and Ownership)
    println!("\n\n\nlesson 4 (Memory Management and Ownership)");
    //// User input
    println!("\nUser input");
    let mut user_data = String::new();
    println!("\nInput something");
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
    // println!("\nOwnership");
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
    println!("\n\n\nlesson 5 (Conditional constructs: if-else, match)");
    // if-else
    println!("\nif-else");
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
    println!("\nTernary operator");
    let condition: bool = true;
    let number: u8 = if condition {5} else {10};
    println!("Ternary operator res: {}", number);

    // Operator match
    println!("\nOperator match");
    let number: u8 = 3;
    
    match number {
        1 => println!("Result 1"),
        2 => println!("Result 2"),
        3 => println!("Result 3"),
        4 => println!("Result 4"),
        5 => println!("Result 5"),
        _ => println!("Else")
    }
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(", World");
}