struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

fn main() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000,
    };

    let car_year = my_car.year;
    my_car.fuel_level = 30.0;
    let extracted_owner = &mut my_car.owner;
    *extracted_owner = String::from("DEF");
    println!("extracted_owner is: {}", extracted_owner);
    println!("Owner is: {}", my_car.owner);

    let another_car = Car {
        owner: "new_name".to_string(),
        ..my_car
    };

    /* Whether I comment owner on line 24 this line below doesn't
    compiles because only stack values are copied, heap values
    are moved to new references */
    //println!("Owner is: {}", my_car.owner);

    // Tuple structs
    let point_2D = (1, 3);
    let point_3D = (4, 10, 13);

    struct Point_2D(i32, i32);
    struct Point_3D(i32, i32, i32);

    let point1 = Point_2D(1, 3);
    let point2 = Point_3D(4, 10, 13);

    // Unit Struct
    struct ABC;
}
