enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn print_ip(ip: IpAddr) {
    match ip.kind {
        IpAddrKind::V4 => println!("IPV4: {}", ip.address),
        IpAddrKind::V6 => println!("IPV6: {}", ip.address)
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    print_ip(home);
    print_ip(loopback);
}
