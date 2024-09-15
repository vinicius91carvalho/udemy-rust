use std::vec;

fn main() {
    let mut vec_1 = vec![4, 5, 6];
    let ref1 = &vec_1;
    let ref2 = &vec_1;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    let ref3 = &mut vec_1;

    let vec_2 = {
        let vec_3 = vec![1, 2, 3];
        // &vec_3
        vec_3
    };
    println!("vec_2: {:?}", vec_2);

    let tuple = (String::from("Hello"), 42);
    // let (s, num) = tuple;
    let (s, num) = &tuple;
    let (s, num) = tuple;
}
