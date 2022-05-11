mod functions;
mod structs_enums;
mod traits;
mod trait_with_option;


fn main() {

    println!("Functions");
    functions::functions();
    println!("\n");


    println!("Struct");
    structs_enums::structs_enums();
    println!("\n");

    println!("Traits");
    traits::traits();
    println!("\n");

    println!("Trait with Option");
    trait_with_option::trait_with_option();
    println!("\n");

}
