// Topic: Result
//
// Requirements:
// * Create an structure named Adult that represents a person aged 21 or older:
//   * The structure must contain the person’s name and age
//   * Implement Debug print functionality using derive
// * Implement a new function for the Adult structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two Adult structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use match to print out a message for each Adult:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u32,
}

impl Adult {
    fn new(name: &str, age: u32) -> Result<Self, String> {
        if age > 20 {
            Ok(Self {
                name: name.to_owned(),
                age,
            })
        } else {
            Err("Sorry!, your age must be 21 or older".to_owned())
        }
    }
}

fn main() {
    let adult_under21 = Adult::new("Samye", 17);
    match adult_under21 {
        Ok(adult) => println!("Welcom {} tp adult club", adult.name),
        Err(msg) => println!("{}", msg),
    }

    let adult_over21 = Adult::new("Sanaa", 23);
    match adult_over21 {
        Ok(adult) => println!("Welcom {} to adult club", adult.name),
        Err(msg) => println!("{}", msg),
    }
}
