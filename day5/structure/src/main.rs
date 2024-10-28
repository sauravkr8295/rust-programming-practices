struct User{
    active : bool,
    user_id : u64,
    username:String,
    email:String,
    sign_in_count:u64,
}
fn main() {
    let user1=User{
        active:true,
        user_id:101,
        username:String::from("Saurav"),
        email:String::from("saurav@gmail.com"),
        sign_in_count:20,
    };
    let email=user1.email;
    println!("{email}");
}
