use std::io;

#[derive(Debug)]
enum Gender {
    Male,
    Female,
    Unknown,
}

struct Human {
    name: String,
    age: u8,
    gender: Gender,
}

fn main() {
    let mut age = String::new();
    let mut name = String::new();

    // Request for name
    println!("What is your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    // Request for age
    println!("What is your age?");
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    let age: u8 = age.trim().parse().expect("Please type a number");

    // Request for gender until valid
    let gender: Gender = loop {
        let mut gender = String::new();

        println!(
            "Please specify your gender in digits
            0 - Unspecified
            1 - Male
            2 - Female"
        );
        io::stdin()
            .read_line(&mut gender)
            .expect("Failed to read line");

        let gender: Option<Gender> = match gender.trim() {
            "0" => Some(Gender::Unknown),
            "1" => Some(Gender::Male),
            "2" => Some(Gender::Female),
            _ => {
                println!(
                    "Gender provided of {} is not valid. Please insert again.",
                    gender.trim()
                );
                None
            }
        };

        match gender {
            Some(gender) => break gender,
            None => continue,
        }
    };

    let human1 = Human {
        name: name.to_string(),
        age,
        gender,
    };

    println!(
        "Welcome {}!\nAge: {}\nGender: {:?}",
        human1.name, human1.age, human1.gender,
    );
}
