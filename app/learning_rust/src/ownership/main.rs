fn main() {
    let s1 = String::from("world");
    {
        let s2 = s1;
    }
    // It doesn't work because the reference to value on heap was pointed to new s2 on stack
    //println!("s1 is: {s1}");

    let x = 15;
    let y = x;
    // It works becasue primitive values are stored only in the stack memory area
    println!("x is: {x}");
}
