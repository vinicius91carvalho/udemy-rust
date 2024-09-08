fn main() {
    // &str and String
    let string_literal: &str = "Hello, World!";
    println!("The value of string_literal is: {}", string_literal);

    let mut flexible_string: String = String::from("Hello, ");
    flexible_string.push_str("World!");
    println!("The value of flexible_string is: {}", flexible_string);

    // Arrays
    let array_1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array_1: {:?}", array_1);
    println!("The value of array_1[3] is: {}", array_1[3]);

    let array_2 = [0; 10];
    println!("array_2: {:?}", array_2);

    // Vectors
    let vec_1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("The value of vec_1[3] is: {}", vec_1[3]);

    // Tuples
    let tuple: (i32, f64, char, &str) = (10, 3.14, 'A', "Hello world!");
    println!("The value of tuple.0 is: {}", tuple.0);
    let (a, b, c, d) = tuple;
    println!(
        "The value of a is: {}, b is: {}, c is: {}, d is: {}",
        a, b, c, d
    );

    let unit: () = ();
    println!("The value of unit is: {:?}", unit);
}
