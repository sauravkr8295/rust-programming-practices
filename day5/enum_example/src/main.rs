enum IpAddrKind{
    V4,
    V6,
}

// fn main() {
//     let four=IpAddrKind::v4;
//     let six=IpAddrKind::v6;
//     route(IpAddrKind::v4);
//     route(IpAddrKind::v6);
// }

// fn route(ip_kind : IpAddrKind){

// }

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

fn main(){
    let home=IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback=IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1");
    }
    

}