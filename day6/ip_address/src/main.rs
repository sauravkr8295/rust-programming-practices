#[derive(Debug)]
enum ipAddrKind{
    v4(String),
    v6(String),
}

// struct ipAddr{
//     kind:ipAddrKind,
//     address:String,
// }


fn main() {
    let four=ipAddrKind::v4;
    let six=ipAddrKind::v6;

    let home=ipAddrKind::v4(String::from("128.0.0.1"));
    let office=ipAddrKind::v6(String::from("::1");)

    // let home=ipAddr{
    //     kind:ipAddrKind::v4,
    //     address:String::from("128.0.0.1"),
    // };

    // let office=ipAddr{
    //     kind:ipAddrKind::v6,
    //     address:String::from("::1"),
    // };


    
}
