#[derive(Debug)]
struct Person<'a> {
    firstname: &'a str,
    lastname: &'a str,
}

fn main() {
    let p;
    {
        let firstname = "Ali";
        let lastname = "Hasani";
        p = Person {
            firstname,
            lastname,
        };

        println!("firstname: {}, lastname: {}", firstname, lastname);
    }
    println!("{:?}", p);
}
