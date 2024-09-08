fn main() {
    // Definition
    let x: i16 = 10;
    println!("The value of x is: {}", x);

    // Mutability
    let y: i32 = 5;
    // y = 6; This will throw an error

    // Scope
    {
        let z: i32 = 50;
    }
    //let s = z; This will throw an error

    // Shadowing
    let t: i32 = 10;
    let t = t + 5;
    println!("The value of t is: {}", t);

    let u = 3;
    let u = 3.0;

    let v = 30;
    {
        let v = 40;
        println!("Inner v is: {}", v);
    }
    println!("Outside v is: {}", v);

    // Constants
    const MAX_POINTS: u32 = 100_000;
}
