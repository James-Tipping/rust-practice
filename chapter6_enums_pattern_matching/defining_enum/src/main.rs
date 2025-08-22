fn main() {
    let my_ip1 = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("192.168.1.1"),
    };

    let my_ip2 = IpAddress2::V4((192, 168, 1, 1));
    println!("Ip Address 2 with pure enum: {:#?}", my_ip2);
}

enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddress2 {
    V4((u8, u8, u8, u8)),
    V6(String),
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}
