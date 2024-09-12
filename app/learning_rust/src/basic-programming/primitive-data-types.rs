fn main() {
    // Unsigned integers
    let unsigned_num: u8 = 255;
    println!("The value of unsigned_num is: {}", unsigned_num); // u16, u32, u64, u128, usize

    // Signed integers
    let signed_num: i8 = -128; // i16, i32, i64, i128, isize
    println!("The value of signed_num is: {}", signed_num);

    // Floating-point numbers
    let float_num: f32 = 3.14; // f64
    println!("The value of float_num is: {}", float_num);

    // Platform speciric integers
    let platform_unsigned_num: usize = 5; // usize
    let platform_signed_num: isize = 5; // usize
    println!(
        "The value of platform_unsigned_num is: {} and platform_signed_num is: {}",
        platform_unsigned_num, platform_signed_num
    );

    // Characters
    let character: char = 'A';
    println!("The value of character is: {}", character);

    // Boolean
    let is_true: bool = true;
    println!("The value of is_true is: {}", is_true);

    // Type aliasing
    type Age = u8;
    let age: Age = 25;
    println!("The value of age is: {}", age);

    // Type conversion
    let a: i32 = 5;
    let b: f64 = a as f64;
    println!("The value of b is: {}", b);
}
