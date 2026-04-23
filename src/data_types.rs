
pub fn data_types_rust() {
    simple_types();
    complex_types();
}

fn simple_types() {
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
}

fn complex_types() {
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

}
