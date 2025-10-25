//program to find roots of quadratic equation 
//and determine its discriminants

use std::io;

//declare input and variables

fn main() {
    println!("Quadratic Equation Calculator");
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().trim().parse().expect("Not a valid number");

    println!("Enter value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().trim().parse().expect("Not a valid number");

    println!("Enter value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().trim().parse().expect("Not a valid number");

    //determine discriminant
    let discriminant = b * b - 4.0 * a * c;
    println!("Discriminant  = {}",discriminant);

    //assign if else statements
    if discriminant > 0.0 {
        let root1 = ( -b + discriminant.sqrt()) / (2.0 * a);
        let root2 = ( -b + discriminant.sqrt()) / (2.0 * a);
        println!("The equation has two distinct real roots");
        println!("Root 1 = {:.3}",root1);
        println!("Root 2 = {:.3}",root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a); 
        println!("The equation has one real root");
        println!("Root = {:.3}",root);
    } else {
        println!("This equation has no real roots");

    }
}
