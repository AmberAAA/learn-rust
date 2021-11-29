enum IpAddKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}




fn main() {
    println!("Hello, world!1");

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1")
    };

    println!("IP: {:?}", home.address)
    
}
