use std::collections::HashMap;

fn main() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
    // let mut vec_1_iter = vec_1.iter_mut();
    // let value_1 = vec_1_iter.next();

    for values in &mut vec_1 {
        println!("{values}");
    }

    println!("{:?}", &mut vec_1);

    let mut person: HashMap<String, i32> = HashMap::new();
    person.insert("Hannash".to_string(), 40);
    person.insert("Joseph".to_string(), 44);
    person.insert("Sara".to_string(), 55);

    for (name, age) in person {
        println!("The person {} has an age of {}", name, age);
    }
}
