use std::fs::File;
use std::io::Write;

fn main() {
    //Nigerian Breweries drink categories
    let categories = ["Lager", "Stout", "Non-alchoholics", "Spirit",];

    //Create the file
    let mut file = File::create("nb_drink_categories.txt").expect("Unsable to create file");

    //Write each category into the file
    for item in categories.iter() {
        writeln!(file, "{}", item).expect("Unable to write to file");
    }

    println!("Drink Categories Saved Successfully!");
}
