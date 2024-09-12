fn main() {
    let num = 40;
    if num < 50 {
        println!("Less than 50");
    } else {
        println!("Greater than or equal to 50");
    }

    let marks = 95;

    let grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else if marks >= 60 {
        'D'
    } else {
        'F'
    };
    println!("Grade: {}", grade);

    let marks = 68;
    let grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("Grade: {}", grade);
}
