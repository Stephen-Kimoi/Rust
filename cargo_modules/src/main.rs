
mod house {
    pub const HOUSE_NUM: u32 = 1; 

    pub mod bedroom_one {
        pub fn is_light_on() -> bool {
            false 
        }

        pub fn is_neighbors_light_on() -> bool {
            use super::bedroom_two; 
            bedroom_two::is_light_on()
        }
    } 

    pub mod bedroom_two {
        pub fn is_light_on() -> bool {
            true
        }
    }

} 

// Modules in a different file
mod house_two; 

use house_two::{bedroom_three, bedroom_four,HOUSE_NUM}; 

// Modules in a different folder 
mod house_three; 

use house_three::{bedroom5, bedroom6, HOUSE3_NUM}; 

fn main() {
    // Using modules in the same file
    println!("====MODULES IN THE SAME FILE: House one==="); 
    println!("House number is {}", house::HOUSE_NUM); 
    println!("Is bedroom one light is on? {}", house::bedroom_one::is_light_on()); 
    println!("Is beroom two light on? {}", house::bedroom_two::is_light_on()); 
    println!("Is neighbors light on? {}", house::bedroom_one::is_neighbors_light_on()); 

    // Using modules in a different file 
    println!("===MODULES IN DIFFERENT FILE: House two==="); 
    println!("House number is {}",HOUSE_NUM);
    println!("Is bedroom three light on? {}",bedroom_three::is_light_on());
    println!("Is bedroom four light on ? {}",bedroom_four::is_light_on());
    println!("Is neighbors light in house two on ? {}",bedroom_three::is_neighbors_light_on());

    // Using modules in a different directory 
    println!("===MODULES IN DIFFERENT FILE: House two==="); 
    println!("House number is {}",HOUSE3_NUM);
    println!("Is bedroom three light on? {}",bedroom5::is_light_on());
    println!("Is bedroom four light on ? {}",bedroom6::is_light_on());
    // println!("Is neighbors light in house two on ? {}",bedroom_five::is_neighbors_light_on());
}