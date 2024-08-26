// Enums allow us to use a list of variants of the same variable. For example, we can IPs with version 4 and 6.
// #[derive(Debug)]
// enum IpAddrKind {
//     IPv4,
//     IPv6,
// }

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// If we want to specify the IP address as well, instead of using structs, We can concise the code by including the strings inside the above enum as following:
#[derive(Debug)]
enum IpAddrKind {
    IPv4(String),
    IPv6(String),
}

// We can also have methods defined for our enums similar to the structs.
impl IpAddrKind {
    fn just_a_function() {
        println!("Does something here!")
    }
}

fn main() {
    // let four = IpAddrKind::IPv4;
    // let six = IpAddrKind::IPv6;

    // let localhost = IpAddr {
    //     kind: IpAddrKind::IPv4,
    //     address: String::from("127.0.0.1"),
    // };
    // println!("{:#?}", localhost);

    let ipaddress = IpAddrKind::IPv4(String::from("127.0.0.1"));
}

fn route(ip_kind: IpAddrKind) {}
