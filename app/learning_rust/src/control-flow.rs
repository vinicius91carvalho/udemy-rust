fn main() {
    'outer: loop {
        println!("Simple loop");
        break 'outer;
    }

    let a = loop {
        break 5;
    };
    println!("The value of a is: {}", a);

    let vec = vec![10, 20, 30, 40, 50];
    for i in vec {
        println!("The value of i is: {}", i);
    }

    let mut num = 0;
    while num < 10 {
        num += 1;
    }
    println!("The value of num is: {}", num);
}
