// Project I – Ownership, Borrowing, Structs, Methods

// Define a structure
struct Student {
    name: String,
    age: u32,
    score: u32,
}

impl Student {
    // Method: calculates if student passed
    fn has_passed(&self) -> bool {
        self.score >= 50
    }

    // Method: update score (mutable reference)
    fn update_score(&mut self, new_score: u32) {
        self.score = new_score;
    }
}

// Function that takes ownership of Student
fn print_details(student: Student) {
    println!("--- Student Details ---");
    println!("Name: {}", student.name);
    println!("Age: {}", student.age);
    println!("Score: {}", student.score);
    println!("Passed: {}", student.has_passed());
    // student goes out of scope here (ownership ends)
}

fn main() {
    // Create a student struct (ownership)
    let mut student1 = Student {
        name: String::from("Femi"),
        age: 17,
        score: 45,
    };

    // Borrowing immutably
    println!("Initial score: {}", student1.score);

    // Borrowing mutably to update score
    student1.update_score(72);

    // Transfer ownership to function
    print_details(student1);

    // student1 can no longer be used here (ownership moved)
    // Uncommenting the next line will cause a compile-time error:
    // println!("{}", student1.score);
}