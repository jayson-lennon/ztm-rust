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

enum Dpt {
    Maintenance,
    Marketing,
    Management,
    LineSupervisor,
    Kitchen,
    AssemblyTech,
}

struct Employee {
    dpt: Dpt,
    terminated: bool,
}

fn try_enter(employee: Employee) -> Result<(), String> {
    if employee.terminated {
        return Err("Terminated employees cannot enter the building".to_owned());
    } else {
        match employee.dpt {
            Dpt::Maintenance => Ok(()),
            Dpt::Marketing => Ok(()),
            Dpt::Management => Ok(()),
            _ => {
                return Err(
                    "Does not belong to a department allowed to enter the building".to_owned(),
                )
            }
        }
    }
}

fn print_can_enter(employee: Employee) -> Result<(), String> {
    try_enter(employee)?;
    println!("Access OK");
    Ok(())
}

fn main() {
    let employees = vec![
        Employee {
            dpt: Dpt::LineSupervisor,
            terminated: false,
        },
        Employee {
            dpt: Dpt::LineSupervisor,
            terminated: true,
        },
        Employee {
            dpt: Dpt::Management,
            terminated: true,
        },
        Employee {
            dpt: Dpt::Management,
            terminated: false,
        },
    ];

    for employee in employees {
        let can_enter = print_can_enter(employee);

        match can_enter {
            Err(e) => println!("{:?}", e),
            _ => println!("Access OK"),
        }
    }
}
