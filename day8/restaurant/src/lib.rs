pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


pub mod front_of_house;
pub use crate::front_of_house::hosting;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_waitlist() {
        hosting::add_to_waitlist();
    }
}



// pub fn eat_at_restaurant{
//     crate::front_of_house::hosting::add_to_waitlist();
// }

// use crate::front_of_house::hosting;

// pub fun eat_at_restaurant(){
//     hosting::add_to_waitlist();
// }