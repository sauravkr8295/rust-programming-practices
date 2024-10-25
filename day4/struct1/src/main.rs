struct User{
    user_id: i64,
    username: String,
    email:String,
    mobile:u64,
}
fn main() {
    let user=User{
        user_id : 101,
        username : String::from("Saurav"),
        email : String::from("sau@gmail.com"),
        mobile : 9029313021,
    };
    let id=user.user_id;
    let email=user.email;
    let mobile=user.mobile;
    println!("user_id of the {} is: {id}",user.username);
    println!("Email_Id of the {} is: {email}",user.username);
    println!("Mobile number of the {} is: {mobile}",user.mobile);
}
