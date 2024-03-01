mod structs;

use std::{collections::HashMap, fs};
use structs::{date::Date, student::Student};

fn main() {
    let mut students = Vec::<Student>::new();
    let content = fs::read_to_string("data.csv").expect("Found and error");
    for line in content.split('\n') {
        let values = line.split(',').collect::<Vec<&str>>();
        students.push(Student::new(
            values[0],
            values[1].parse().expect("Not able to parse the message"),
            values[2],
            Date::parse(values[3]),
            get_marks(values[4]),
        ));
    }
    for student in students {
        println!("{} -> {}%", student.name, student.get_percentage());
    }
}

fn get_marks(marks_str: &str) -> HashMap<String, f32> {
    let marks = marks_str
        .split('|')
        .into_iter()
        .map(|m| m.parse::<f32>().expect("Error parsing marks"))
        .collect::<Vec<f32>>();
    HashMap::from([
        (String::from("Maths"), marks[0]),
        (String::from("Physics"), marks[1]),
        (String::from("Chemistry"), marks[2]),
    ])
}
