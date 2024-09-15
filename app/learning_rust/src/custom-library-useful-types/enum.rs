// enum WeekDay {
//     Sunday,
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
// }

// fn main() {
//     let mut day = "Saturday".to_string();

//     let week_days = vec![
//         "Sunday".to_string(),
//         "Monday".to_string(),
//         "Tuesday".to_string(),
//         "Wednesday".to_string(),
//         "Thursday".to_string(),
//         "Friday".to_string(),
//         "Saturday".to_string(),
//     ];

//     day = week_days[1].clone();

//     let day = WeekDay::Saturday;
// }

enum TravelType {
    Car(f32),
    Train(f32),
    Aeroplace(f32),
}

impl TravelType {
    fn travel_allowance(&self) -> f32 {
        match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Aeroplace(miles) => miles * 5.0,
        }
    }
}

fn main() {
    let participant = TravelType::Car(60.0);
    println!("Travel allowance: {}", participant.travel_allowance());
}
