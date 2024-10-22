fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    // let mut result: Vec<String> = vec![];

    // for word in words {
    //     if word.starts_with("a") || word.starts_with("b") {
    //         let uppercase_word = word.to_uppercase();
    //         result.push(uppercase_word);
    //     }
    // }

    let result: Vec<String> = words
        .into_iter()
        .filter(|word| word.starts_with("a") || word.starts_with("b"))
        .map(|word| word.to_uppercase())
        .collect();

    println!("{:?}", result);

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut result = 0;

    let result: i32 = numbers
        .iter()
        .filter(|&&num| num % 2 != 0)
        .map(|&num| num * num)
        .sum();

    println!("Result without combinators: {}", result);
}
