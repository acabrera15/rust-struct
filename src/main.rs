use std::io;
struct Employee {
    name: String,
    id: u32,
    department: String,
    salary: f64,
}

fn main() {
    let mut employees = vec![Employee {
        name: String::from("Andrew"),
        id: 12,
        department: String::from("Business"),
        salary: 10000.00,
    }];
    println!("welcome to my employee program");
    println!("Employee Database");
    println!("-----------------");
    loop {
        // Options
        println!("1. Add Employee");
        println!("2. List All Employees");
        println!("3. Update Employee");
        println!("4. Filter Employees by Department");
        println!("5. Exit");

        // Handle user input
        let mut user_option = String::new();
        io::stdin()
            .read_line(&mut user_option)
            .expect("Please input a number ");

        let user_option: u8 = match user_option.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 5 {
                    println!("Enter values 1-5");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Enter a value 1-5");
                continue;
            }
        };

        if user_option == 1 {
            employees.push(create_employee());
        } else if user_option == 2 {
            list_employees(&employees);
        } else if user_option == 3 {
            update_employee(&mut employees);
        } else if user_option == 4 {
            list_by_department(&employees);
        } else if user_option == 5 {
            println!("Goodbye!");
            return;
        }
    }
}

fn create_employee() -> Employee {
    // capture name
    println!("Enter employee's name");
    let mut employee_name = String::new();
    io::stdin()
        .read_line(&mut employee_name)
        .expect("Unable to read line");

    // capture department namecreate
    println!("\nEnter department name");
    let mut department_name = String::new();
    io::stdin()
        .read_line(&mut department_name)
        .expect("Unable to read line");

    // capture salary
    let salary_num: f64;
    loop {
        println!("\nEnter employee's salary:");
        let mut salary = String::new();
        io::stdin()
            .read_line(&mut salary)
            .expect("Unable to read line");
        match salary.trim().parse() {
            Ok(num) => salary_num = num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        break;
    }

    // return employee
    Employee {
        name: employee_name,
        department: department_name,
        salary: salary_num,
        id: 123,
    }
}

fn list_employees(employees: &Vec<Employee>) {
    for employee in employees {
        println!(
            "Name: {}, ID: {}, Salary: {}, Department: {}",
            employee.name, employee.id, employee.salary, employee.department
        )
    }
}

fn update_employee(employees: &mut Vec<Employee>) {
    let mut employee_id = String::new();
    let employee_id_num: u32;
    // get use input
    loop {
        println!("Enter employee's id you would like to update");
        io::stdin()
            .read_line(&mut employee_id)
            .expect("Unable to read line");

        match employee_id.trim().parse() {
            Ok(num) => employee_id_num = num,
            Err(_) => {
                println!("Please enter a valid numbed");
                continue;
            }
        }

        break;
    }

    if let Some(found_employee) = employees.iter_mut().find(|x| x.id == employee_id_num) {
        println!("Enter the number of the value you like to update?");
        println!("1. name: {}", found_employee.name);
        println!("2. department: {}", found_employee.department);
        println!("3. salary: {}", found_employee.salary);

        let mut item_to_change = String::new();
        let mut item_to_change_num: u8;

        loop {
            io::stdin()
                .read_line(&mut item_to_change)
                .expect("Unable to read line");

            match item_to_change.trim().parse() {
                Ok(num) => item_to_change_num = num,
                Err(_) => {
                    println!("Please enter a valid number");
                    continue;
                }
            }

            if item_to_change_num > 3 || item_to_change_num < 1 {
                println!("Enter a value from 1-3");
                continue;
            }

            break;
        }

        if item_to_change_num == 1 {
            println!("Enter a new name");
            let mut new_name = String::new();

            io::stdin()
                .read_line(&mut new_name)
                .expect("Unable to read line");

            found_employee.name = new_name;
        } else if item_to_change_num == 2 {
            println!("Enter a new department");
            let mut new_department = String::new();
            io::stdin()
                .read_line(&mut new_department)
                .expect("Unable to read line");

            found_employee.department = new_department;
        } else if item_to_change_num == 3 {
            let new_salary_num: f64;

            loop {
                println!("Enter a new salary");
                let mut new_salary = String::new();

                io::stdin()
                    .read_line(&mut new_salary)
                    .expect("Unable to read line");

                match new_salary.trim().parse() {
                    Ok(num) => new_salary_num = num,
                    Err(_) => {
                        println!("Please enter a valid number");
                        continue;
                    }
                }
                break;
            }

            found_employee.salary = new_salary_num;
        }

        println!("The new employee looks like: ");
        println!(
            "name: {} id: {} department: {} salary: {}",
            found_employee.name,
            found_employee.id,
            found_employee.department,
            found_employee.salary
        )
    } else {
        println!("An employee with id: {} as not found. Please look at the employee list to use a valid ID", employee_id);
    }
}

fn list_by_department(employees: &Vec<Employee>) {
    let mut entered_department = String::new();

    io::stdin()
        .read_line(&mut entered_department)
        .expect("Unable to read string");

    let entered_department = entered_department.trim().to_ascii_lowercase();

    let mut found_any = false;

    for employee in employees {
        if employee.department.to_ascii_lowercase() == entered_department {
            println!("Name: {}", employee.name);
            println!("ID: {}", employee.id);
            println!("Department: {}", employee.department);
            println!("Salary: {}", employee.salary);
            found_any = true;
        }
    }

    if !found_any {
        println!("No employees found with department: {}", entered_department);
    }
}
