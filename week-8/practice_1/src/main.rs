fn  main() {
    //Using Vex::new()
    let v : Vex<i64> = Vec::new();

    //printing the size of a vector
    println!("\n The length of Vex::new() is {}", v.len());

    //using macro
    let vec1 = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    //printing the size of vector
    println!("\nThe length of vec macro is: {}",v.len() );
}