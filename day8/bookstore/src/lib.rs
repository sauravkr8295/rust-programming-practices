pub mod bookstore;

pub use crate::bookstore::inventory::add_book;
pub use crate::bookstore::customers::add_customer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_book(){
        add_book("Rust Programming","Steve");
    }

    #[test]
    fn test_add_customer(){
        add_customer("Alice");
    }
    
}
