fn main() {
    // comment

    /* 
        Multi-line
        comment
    */ 
    println!("Hello, world!");

    // Integer: i8; i16; i32; i64; i128; u8; u16; u32; u64; u128
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
