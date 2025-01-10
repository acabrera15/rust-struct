use std::io;
struct Employee {
    name: String,
    id: u32,
    department: String,
    salary: f64,
}

struct Department {
    name: String,
    employees: Vec<Employee>,
}
// fn main() {
//     let departments: Vec<Department>;
//     println!("welcome to my employee program");
//     println!("Employee Database");
//     println!("-----------------");
//     loop {
//         // Options
//         println!("1. Add Employee");
//         println!("2. List All Employees");
//         println!("3. Update Employee");
//         println!("4. Filter Employees by Department");
//         println!("5. Exit");

//         // Handle user input
//         let mut user_option = String::new();
//         io::stdin()
//             .read_line(&mut user_option)
//             .expect("Please input a number ");

//         let user_option: u8 = match user_option.trim().parse() {
//             Ok(num) => {
//                 if num < 1 || num > 5 {
//                     println!("Enter values 1-5");
//                     continue;
//                 } else {
//                     num
//                 }
//             }
//             Err(_) => {
//                 println!("Enter a value 1-5");
//                 continue;
//             }
//         };
//     }
// }

fn main() {
    create_employee();
}
fn create_employee() -> Employee {
    // capture name
    println!("Enter employee's name");
    let mut employee_name = String::new();
    io::stdin()
        .read_line(&mut employee_name)
        .expect("Unable to read line");

    // capture department name
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
