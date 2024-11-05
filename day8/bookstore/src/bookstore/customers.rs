 pub struct Customer{
    pub name: String,
 }

 pub fn add_customer(name: &str){
    let new_customer=Customer{
        name: name.to_string(),
    };
    println!("New customer {} added",new_customer.name);
}
