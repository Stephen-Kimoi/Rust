pub struct Counter {
    val: i8, 
}

impl Counter {
    pub fn get_number(&self) -> i8 {
        return self.val; 
    } 

    // Increament from the counter
    pub fn increament(&mut self) {
        self.val = self.val + 1; 
        let log_message = format!("Increased number to {}", self.val); 
        println!("{}", log_message.to_string()); 
    } 

    // Decreament from the counter 
    pub fn decreament(&mut self) {
        self.val -= self.val; 
        let log_message = format!("Decreased number to {}", self.val); 
        println!("{}", log_message.to_string()); 
    }
}

fn main() {
    let mut counter = Counter {
        val: 0 
    }; 
    counter.increament(); 
    println!("After increamenting the counter is {}", counter.get_number())
}