struct DrawingInfo {
    line_width: u8,
    color: String,
}

struct Square {
    side: f32,
    info: DrawingInfo,
}

struct Rectangle {
    length: f32,
    width: f32,
    info: DrawingInfo,
}

struct Circle {
    radius: f32,
}

// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

trait Draw {
    fn draw_object(&self);
}

trait OtherTrait {}
impl OtherTrait for Square {}
impl OtherTrait for Rectangle {}

trait SomeOtherTrait {}
impl SomeOtherTrait for Square {}
impl SomeOtherTrait for Rectangle {}

trait Shape: Draw + OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }

    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("Rectangle Perimeter: {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!(
            "Drawing a rectangle with length: {} and width: {}",
            self.length, self.width
        );
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing a square with side: {}", self.side);
    }
}

// fn shape_properties<T: Shape>(object: T) {
//     object.area();
//     object.perimeter();
// }

// fn shape_properties(object: impl Shape) {
//     object.area();
//     object.perimeter();
// }

fn shape_properties_rect(object: Rectangle) {
    object.area();
    object.perimeter();
}

fn shape_properties_sq(object: Square) {
    object.area();
    object.perimeter();
}

fn shape_properties_static<T>(object: T)
where
    T: Shape,
{
    object.area();
    object.perimeter();
}

fn shape_properties_dynamic(object: Box<dyn Shape>) {
    object.area();
    object.perimeter();
}

fn returns_shape(dimension: Vec<f32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        let sq = Square {
            side: dimension[0],
            info: DrawingInfo {
                line_width: 5,
                color: String::from("Red"),
            },
        };
        Box::new(sq)
    } else {
        let req = Rectangle {
            length: dimension[0],
            width: dimension[1],
            info: DrawingInfo {
                line_width: 5,
                color: String::from("Red"),
            },
        };
        Box::new(req)
    }
    // let sq = Square {
    //     side: 5.0,
    //     info: DrawingInfo {
    //         line_width: 5,
    //         color: String::from("Red"),
    //     },
    // };
    // sq
    // let rect = Rectangle {
    //     length: 5.0,
    //     width: 10.0,
    //     info: DrawingInfo {
    //         line_width: 5,
    //         color: String::from("Red"),
    //     },
    // };

    // let x = false;

    // if x {
    //     sq
    // } else {
    //     rect
    // }
}

fn main() {
    let r1 = Rectangle {
        width: 5.0,
        length: 4.0,
        info: DrawingInfo {
            line_width: 1,
            color: String::from("Red"),
        },
    };

    let s1 = Square {
        side: 3.2,
        info: DrawingInfo {
            line_width: 1,
            color: String::from("Red"),
        },
    };

    r1.area();
    s1.area();

    r1.perimeter();
    s1.perimeter();

    let c1 = Circle { radius: 5.0 };
    shape_properties_dynamic(Box::new(r1));
    shape_properties_dynamic(Box::new(s1));
    //shape_properties(c1);
}
