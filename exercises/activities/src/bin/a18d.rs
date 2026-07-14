// Topic: Result & the question mark operator

// Summary:
//   This small program simulates unlocking a door using digital keycards
//   backed by a database. Many errors can occur when working with a database,
//   making the question mark operator the perfect thing to use to keep
//   the code manageable.
//
// Requirements:
// * Write the body of the `authorize` function. The steps to authorize a user
//   are:
//      1. Connect to the database
//      2. Find the employee with the `find_employee` database function
//      3. Get a keycard with the `get_keycard` database function
//      4. Determine if the keycard's `access_level` is sufficient, using the
//         `required_access_level` function implemented on `ProtectedLocation`.
//         * Higher `access_level` values grant more access to `ProtectedLocations`.
//           1000 can access 1000 and lower. 800 can access 500 but not 1000, ...
// * Run the program after writing your `authorize` function. Expected output:
//     Ok(Allow)
//     Ok(Deny)
//     Err("Catherine doesn't have a keycard")
// * Use the question mark operator within the `authorize` function.
//
// Notes:
// * Only the `authorize` function should be changed. Everything else can remain
//   unmodified.

#[derive(Debug, Copy, Clone)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

struct Employee {
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        Ok(Self)
    }

    fn find_employee(&self, emp_name: &str) -> Result<Employee, String> {
        match emp_name {
            "ali" | "ahmad" | "nadya" => Ok(Employee {
                name: emp_name.to_string(),
            }),
            _ => Err(format!("Employee with name {} is not found", emp_name)),
        }
    }

    fn get_keycard(&self, emp: &Employee) -> Result<KeyCard, String> {
        if emp.name == "ali" {
            Ok(KeyCard { access_level: 1000 })
        } else if emp.name == "ahmad" {
            Ok(KeyCard { access_level: 800 })
        } else {
            Err(format!("{} doesn't have a keycard", emp.name))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

// Only change this function
fn authorize(emp_name: &str, location: &ProtectedLocation) -> Result<AuthorizationStatus, String> {
    let db = Database::connect()?;
    let emp = db.find_employee(emp_name)?;
    let keycard = db.get_keycard(&emp)?;

    if keycard.access_level >= location.required_access_level() {
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    let ali_auth = authorize("ali", &ProtectedLocation::Warehouse);
    match ali_auth {
        Ok(authz) => println!("{:?}", authz),
        Err(msg) => println!("{}", msg),
    }
    let ahmad_auth = authorize("ahmad", &ProtectedLocation::Office);
    match ahmad_auth {
        Ok(authz) => println!("{:?}", authz),
        Err(msg) => println!("{}", msg),
    }

    let nadya_auth = authorize("nadya", &ProtectedLocation::Office);
    match nadya_auth {
        Ok(authz) => println!("{:?}", authz),
        Err(msg) => println!("{}", msg),
    }
}
