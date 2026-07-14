// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

#[derive(Debug)]
enum EmployeeType {
    MaintenanceCrew,
    MarketingDepartmentEmployee,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

struct Employee {
    emp_type: EmployeeType,
    terminated_employment: bool,
}

impl Employee {
    fn new(emp_type: EmployeeType, terminated_employment: bool) -> Self {
        Self {
            emp_type,
            terminated_employment,
        }
    }

    fn may_enter_building(&self) -> Result<(), &str> {
        if self.terminated_employment {
            Err("Sorry!, terminated employees cannot access the building regardless of their position")
        } else {
            Ok(())
        }
    }
}

fn print(emp: &Employee) -> Result<(), &str> {
    emp.may_enter_building()?;

    println!("Access is permitted for {:?}", emp.emp_type);

    Ok(())
}

fn main() {
    let terminated_emp = Employee::new(EmployeeType::LineSupervisor, true);
    match print(&terminated_emp) {
        Ok(_) => {}
        Err(msg) => println!("{msg}"),
    }

    let active_emp = Employee::new(EmployeeType::AssemblyTechnician, false);
    match print(&active_emp) {
        Ok(_) => {}
        Err(msg) => println!("{msg}"),
    }
}
