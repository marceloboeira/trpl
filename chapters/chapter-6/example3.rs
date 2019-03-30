enum IpAddr {
    V4(String),
    V6(String),
}

fn print_ip(ip: IpAddr) {
    match ip {
        IpAddr::V4(addr) => println!("IPV4: {}", addr),
        IpAddr::V6(addr) => println!("IPV6: {}", addr)
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    print_ip(home);
    print_ip(loopback);
}
