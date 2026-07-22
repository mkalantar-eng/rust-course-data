#[derive(Debug)]
struct Person<'a> {
    firstname: &'a str,
    lastname: &'a str,
}

fn main() {
    let p;
    let firstname = "Ali".to_string();
    {
        let lastname = "Hasani".to_string();
        {
            p = Person {
                firstname: &firstname,
                lastname: &lastname,
            };
        }
    }
    println!("{:?}", p);
}
