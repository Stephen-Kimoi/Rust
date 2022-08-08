pub const HOUSE_NUM: u32 = 2; 

    pub mod bedroom_three {
        pub fn is_light_on() -> bool {
            false 
        }

        pub fn is_neighbors_light_on() -> bool {
            use super::bedroom_four; 
            bedroom_four::is_light_on()
        }
    } 

    pub mod bedroom_four {
        pub fn is_light_on() -> bool {
            true
        }
    }