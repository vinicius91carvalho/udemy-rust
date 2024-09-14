fn main() {
    let mut vec_1 = vec![1, 2, 3];
    borrows_vec(&vec_1);
    let ref1 = &mut vec_1;
    mutably_borrows_vec(ref1);
    println!("vec_1 is: {:?}", vec_1);
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("vec: {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10);
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}
