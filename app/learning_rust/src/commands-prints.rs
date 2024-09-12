fn main() {
    // The current line is a comment line
    // This is the second line of comment

    /*
       Multi
       Line
       Comment
    */

    print!("This is a print command");
    print!("This is going to be printed on the same line");

    /* Escape Characters
       \n - New Line
       \t - Tab
       \\ - Backslash
       \" - Double Quote
       \' - Single Quote
       \r - Carriage Return
    */
    println!("This is a println command \t This is a tab \"This is a double quote\" 'This is a single quote' \\ This is a backslash");

    println!(
        "I am doing {2} from {1} years and i {0} it",
        "like", 13, "programming"
    );

    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        language = "Rust",
        activity = "code"
    );

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: f64 = n.trim().parse().expect("invalid input");
    println!("You entered: {}", n);

    let _number_one = 43.0;
    let _x = 40_000;

    static WELCOME: &str = "Welcome to Rust Programming";
    const PI: f64 = 3.14159;

    // a and b are stored in different memory locations
    let _a = PI;
    let _b = PI;

    // c and d are references to the same memory location due to static type
    let _c = WELCOME;
    let _d = WELCOME;
}
