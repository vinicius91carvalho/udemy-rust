fn main() {
    my_fn("This is my function");

    let str = "Function call with a variable";
    my_fn(str);

    let answer = multiplication(10, 15);
    println!("The answer is: {answer}");

    let result = basic_math(10, 15);
    let (multiplication, addition, subtraction) = result;
    println!("Multiplication: {multiplication}");
    println!("Addition: {addition}");
    println!("Subtraction: {subtraction}");

    let full_name: String = {
        let first_name = "Nouman";
        let last_name = "Ahmad";
        format!("{first_name} {last_name}")
    };
    println!("full_name: {:?}", full_name);
}

fn my_fn(s: &str) {
    println!("{s}");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Multiplying {num1} by {num2}");
    num1 * num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
