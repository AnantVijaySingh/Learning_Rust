// Using a hash map and vectors, create a text interface to allow a user to add employee names to a
// department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
// the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn main() {
    let mut employee_database: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("- - - - - - - - - -");
        println!("Please select an option");
        println!("1: to add a new employee");
        println!("2: to list all employees in an department");
        println!("3: to list all employees per department");
        println!("4: fill with mock data");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option:u8 = match option.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please select a valid option");
                continue
            }
        };

        match option {
            1 => add_employee(&mut employee_database),
            2 => employees_per_dept(&employee_database),
            3 => all_employees(&employee_database),
            4 => fill_mock_data(&mut employee_database),
            _ => {}
        }

        println!("To continue press enter/return or type esc to end program");

        let mut continue_esc = String::new();

        io::stdin()
            .read_line(&mut continue_esc)
            .expect("Failed to read line");

        let continue_esc: String = match continue_esc.trim().parse() {
            Ok(input) => input,
            Err(_) => {
                println!("Error");
                continue
            }
        };

        match continue_esc.as_str() {
            "esc" => { break },
            _ => continue,
        }
    }
}


fn employees_per_dept (database: &HashMap<String, Vec<String>>) {

    let mut dept = String::new();

    println!("- - - - - - - - - -");
    println!("Enter department");

    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read line");

    dept = dept.strip_suffix("\n").unwrap_or(&*dept).parse().unwrap();

    let emp_list = database.get(&*dept)
        .cloned()
        .unwrap_or(vec![String::from("Department not found")]);

    for names in emp_list {
        println!("{}", names)
    }

}

fn add_employee (employee_database: &mut HashMap<String, Vec<String>>) {
    let mut name = String::new();
    let mut dept = String::new();

    println!("- - - - - - - - - -");
    println!("Enter employee name");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("- - - - - - - - - -");
    println!("Enter employee department");

    io::stdin()
        .read_line(&mut dept)
        .expect("Failed to read line");

    let name: String = name.strip_suffix("\n").unwrap_or(&*name).parse().unwrap();
    let dept: String = dept.strip_suffix("\n").unwrap_or(&*dept).parse().unwrap();
    let name_copy: String = name.clone();

    employee_database.entry(dept)
        .and_modify(|value| { value.push(name) })
        .or_insert(vec![name_copy]);
}

fn all_employees(database: &HashMap<String, Vec<String>>) {
    println!("{:?}", database);
}

fn fill_mock_data (employee_database: &mut HashMap<String, Vec<String>>) {
    employee_database.entry(String::from("Engineering"))
        .and_modify(|value| { value.push(String::from("Sally")) })
        .or_insert(vec![String::from("Sally")]);

    employee_database.entry(String::from("Product"))
        .and_modify(|value| { value.push(String::from("Jack")) })
        .or_insert(vec![String::from("Jack")]);

    employee_database.entry(String::from("Engineering"))
        .and_modify(|value| { value.push(String::from("Peter")) })
        .or_insert(vec![String::from("Peter")]);

    employee_database.entry(String::from("Sales"))
        .and_modify(|value| { value.push(String::from("Daniel")) })
        .or_insert(vec![String::from("Daniel")]);

    employee_database.entry(String::from("Product"))
        .and_modify(|value| { value.push(String::from("Marissa")) })
        .or_insert(vec![String::from("Marissa")]);
}