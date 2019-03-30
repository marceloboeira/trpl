enum IpAddrKind {
    V4,
    V6,
}

fn print_iptype(ip: IpAddrKind) {
    match ip {
        IpAddrKind::V4 => println!("This is version is 4"),
        IpAddrKind::V6 => println!("This is version is 6")
    }
}

fn main() {
    let iptype1 = IpAddrKind::V4;
    let iptype2 = IpAddrKind::V6;

    print_iptype(iptype1);
    print_iptype(iptype2);
}
