//program to determine the annual incentive of employees

use std::io;


//declaring variables and values
fn main() {
    println!("Annual Incentive Calculator");
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nDoes employee have experience? (yes/no)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let experience = input1.trim().to_lowercase();

//only ask for age if experience = yes
    if experience == "yes" {
    println!("\nEnter Age of the employee: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:u32 = input2.trim().parse().expect("Not a valid number");

    //assigning if else statements for age
        if age >= 40 {
            println!("The Incentive of the employee is ₦1,560,000.");
        } else if age >=30 && age < 40 {
            println!("The Incentive of the employee is ₦1,480,000.");
    } else if age <28 {
            println!("The Incentive of the employee is ₦1,300,000.");
        } else {
            println!("No Incentive is specified for this group of employees.");
        }
    }  //assigning if else statements for experience is no
    else if experience == "no" {
        println!("The Incentive of the employee is ₦100,000");
        } else {
            println!("Invalid input for experience. Please enter 'yes' or 'no'.");
    }
}
