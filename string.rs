struct Student {
    name: String,
    email: String,
    phone: String,
    id: u32,
}

fn main() {
    let students = vec![
        Student {
            name: String::from("Alice"),
            email: String::from("alice@example.com"),
            phone: String::from("123-456-7890"),
            id: 1,
        },
        Student {
            name: String::from("Bob"),
            email: String::from("bob@example.com"),
            phone: String::from("987-654-3210"),
            id: 2,
        },
        Student {
            name: String::from("Charlie"),
            email: String::from("charlie@example.com"),
            phone: String::from("555-555-5555"),
            id: 3,
        },
        Student {
            name: String::from("David"),
            email: String::from("david@example.com"),
            phone: String::from("111-222-3333"),
            id: 4,
        },
        Student {
            name: String::from("Eve"),
            email: String::from("eve@example.com"),
            phone: String::from("999-888-7777"),
            id: 5,
        },
    ];

    let student_index = 2; // Change this to access a different student
    match students.get(student_index) {
        Some(student) => {
            println!("Student Details:");
            println!("Name: {}", student.name);
            println!("Email: {}", student.email);
            println!("Phone: {}", student.phone);
            println!("ID: {}", student.id);
        }
        None => {
            println!("Student not found");
        }
    }
}