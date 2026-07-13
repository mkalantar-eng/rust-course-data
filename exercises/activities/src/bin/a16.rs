// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

#[derive(Debug)]
struct Student {
    name: String,
    locker_assignment: Option<i32>,
}

impl Student {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let student = Student {
        name: "Ali".to_owned(),
        locker_assignment: Some(6),
    };

    student.print();

    match student.locker_assignment {
        Some(n) => println!("locker assignment: {n}"),
        None => println!("No locker assignment!"),
    }
}
