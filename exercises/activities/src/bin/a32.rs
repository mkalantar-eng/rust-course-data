// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

#[derive(Debug)]
struct Person<'a> {
    first_name: &'a str,
    title: &'a str,
}

fn main() {
    let mut persons: Vec<Person> = vec![];
    let mut lines = MOCK_DATA.lines().skip(1);
    while let Some(line) = lines.next() {
        let parts: Vec<_> = line.split(',').collect();
        let person = Person {
            first_name: parts[1],
            title: parts[4],
        };

        println!("{:?}", person);
        persons.push(person);
    }
}
