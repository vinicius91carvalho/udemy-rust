use std::collections::HashMap;
fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();
    person.insert("Nouman", 40);
    person.insert("Kamram", 44);
    person.insert("Shahid", 55);

    println!("The age is {:?}", person.get("Nouman").unwrap());

    if person.contains_key("Nouman") {
        println!("Nouman exists on HashMap.");
    } else {
        println!("Nouman doesn't exists on HashMap.");
    }

    match person.get("Nouman") {
        Some(value) => println!("The value exist {}", value),
        None => println!("The value doesn't exist"),
    }

    for (name, age) in &person {
        println!("The person {} has an age of {}", name, age);
    }

    let mut likes: HashMap<&str, &str> = HashMap::new();
    likes.entry("Nouman").or_insert("apple");
    likes.entry("Nouman").or_insert("banana");

    println!("The fruit which is being liked is {:?}", likes);

    likes.insert("Nouman", "apple");
    likes.insert("Nouman", "mango");

    println!("The new fruit which is being liked is {:?}", likes);

    let some_vec = vec![
        2, 4, 3, 1, 3, 5, 2, 2, 3, 3, 1, 2, 4, 4, 5, 4, 5, 23, 23, 4, 2, 32, 2, 12,
    ];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}", freq_vec);
}
