// Problem 1:
/* In this exercise, you will be working on creating a student management system
using Rust. The system should allow you to store and retrieve student information
based on their unique ID. For ease of work, the student structure is already
created in the code below

Next, create a StudentManager structure containing a field of student, which
will essentially be a hashmap where the key part will be an integer representing
unique ID of student and the value part will be the complete details of the
students contained in the student structure.

The StudentManager should implement the following methods:
1. new() -> Self: A constructor that initializes an empty student manager.

2. add_student(&mut self, student: Student) -> Result<(), String>:
Adds a student to the manager.
If the student's ID already exists, return an error message.
Otherwise, add the student to the manager and return Ok.

3. get_student(&self, id: i32) -> Option<&Student>: Retrieves a student
from the manager based on their ID.
If the student is found, return Some(student). Otherwise, return None.

Your task is to implement the StudentManager structure, and the mentioned methods.
Additionally, provide a sample usage of the student management system by adding
a few students and retrieving their information using the get_student() method.
*/

use std::collections::HashMap;

struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    student_db: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            student_db: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.student_db.contains_key(&student.id) {
            return Err(String::from("Student already exists"));
        }
        self.student_db.insert(student.id, student);
        Ok(())
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        if (self.student_db.contains_key(&id)) {
            return self.student_db.get(&id);
        }
        None
    }
}

fn main() {
    let mut student_manager = StudentManager::new();

    let student_1 = Student {
        id: 1,
        grade: String::from("10"),
        name: String::from("Felipe"),
    };

    let student_2 = Student {
        id: 2,
        grade: String::from("9"),
        name: String::from("Rafaela"),
    };

    let student_3 = Student {
        id: 3,
        grade: String::from("8"),
        name: String::from("Josefina"),
    };

    let student_error = Student {
        id: 1,
        grade: String::from("8"),
        name: String::from("Josefina"),
    };

    let result = student_manager.add_student(student_1);

    match result {
        Ok(()) => println!("Sign up successfully!"),
        Err(err) => println!("{err}"),
    }

    // println!("{:?}", student_1.id); Error because student_1 was borrowed
    if let Ok(()) = student_manager.add_student(student_2) {
        println!("Sign up successfully using if let!");
    }

    student_manager.add_student(student_3);

    if let Err(err) = student_manager.add_student(student_error) {
        println!("{err}");
    }

    let student_1 = student_manager.get_student(1);
    println!("Getting student by id 1: name={}", student_1.unwrap().name);
}
