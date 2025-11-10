use std::io;

fn main() {

    println!("\nWelcome to the Campus Cafe order System Dear Customer!");
    println!(".....................................................");
    println!("|                  Menu               |  | Price(₦)  |");
    println!(".....................................................");
    println!("|  P  |  |  Poundo Yam/Edinkaiko Soup |  |   3,200   |");
    println!(".....................................................");
    println!("|  F  |  |    Fried Rice & Chicken    |  | 3,000 |");
    println!(".....................................................");
    println!("|  A  |  |     Amala & Ewedu Soup     |  |  2,500 |");
    println!(".....................................................");
    println!("|  E  |  |     Eba & Egusi Soup       |  | 2,000  |");
    println!(".....................................................");
    println!("|  W  |  |     White Rice & Stew      |  |  2,500 |");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let price1:u32 = 3_200;
    let price2:u32 = 3_000;
    let price3:u32 = 2_500;
    let price4:u32 = 2_000;
    let price5:u32 = 2_500;

    println!("\n Please enter your desired food item (P,F,A,E,W): ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let order = input1.trim().to_uppercase();

     println!("\nEnter your desired quantity: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let quantity:u32 = input2.trim().parse().expect("Not a valid input");

    if order == "P" {
       let mut total_order = (quantity * price1) as f32;
       if total_order > 10_000.0 {
        let discount = total_order * 0.05;
        total_order = total_order - discount;
        println!("\nA 5% discount has been applied (:.2) naira");
       }
       println!("\nYour order is: {}", order);
        println!("\nYour quantity is: {}", quantity);
        println!("\nYour total order is: {:.2} naira", total_order);
        }else if order == "F" {
       let mut total_order = (quantity * price2) as f32;
      if total_order > 10_000.0 {
        let discount = total_order * 0.05;
        total_order = total_order - discount;
        println!("\nA 5% discount has been applied (:.2) naira");
       }
       println!("\nYour order is: {}", order);
        println!("\nYour quantity is: {}", quantity);
        println!("\nYour total order is: {:.2} naira", total_order);
    }else if order == "A" {
       let mut total_order = (quantity * price3) as f32;
       if total_order > 10_000.0 {
        let discount = total_order * 0.05;
        total_order = total_order - discount;
        println!("\nA 5% discount has been applied (:.2) naira");
       }
       println!("\nYour order is: {}", order);
        println!("\nYour quantity is: {}", quantity);
        println!("\nYour total order is: {:.2} naira", total_order);
    }else if order == "E" {
       let mut total_order = (quantity * price4) as f32;
        if total_order > 10_000.0 {
        let discount = total_order * 0.05;
        total_order = total_order - discount;
        println!("\nA 5% discount has been applied (:.2) naira");
    }   
    println!("\nYour order is: {}", order);
        println!("\nYour quantity is: {}", quantity);
        println!("\nYour total order is: {:.2} naira", total_order); 
    }else if order == "W" {
       let mut total_order = (quantity * price5) as f32;
       if total_order > 10_000.0 {
        let discount = total_order * 0.05;
        total_order = total_order - discount;
        println!("\nA 5% discount has been applied (:.2) naira");
       }
        println!("\nYour order is: {}", order);
        println!("\nYour quantity is: {}", quantity);
        println!("\nYour total order is: {:.2} naira", total_order);
    }else {
        println!("\nEnter an accurate food item please.");
    }
   

    println!("\n\nThank you for the Campus Cafe order Dear Customer!");

}
