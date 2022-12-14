use crate::structs::date::Date;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

pub struct Student {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub dob: Date,
    pub marks: HashMap<String, f32>,
}

impl Student {
    pub fn new(
        name: &str,
        age: i32,
        email: &str,
        dob: Date,
        marks: HashMap<String, f32>,
    ) -> Student {
        Student {
            name: String::from(name),
            age,
            email: String::from(email),
            dob,
            marks,
        }
    }

    pub fn get_percentage(&self) -> f32 {
        self.marks.values().sum::<f32>() * &0.333
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}, age: {}, email: {}, Date of birth: {}",
            self.name, self.age, self.email, self.dob
        )
    }
}

