#[derive(Debug)]
enum IpAddrVer {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    version: IpAddrVer,
    address: String,
}

fn main() {
    let four = IpAddrVer::V4;
    let six = IpAddrVer::V6;
    route(four);
    route(six);

    let home = IpAddr {
        version: IpAddrVer::V4,
        address: String::from("127.0.0.1"),
    };

    println!("My home IP address is {:?}", home);
    println!("My home IP address is {:?} and the version is {}", home.version, home.address);
}

fn route(if_version: IpAddrVer) {
    match if_version {
        IpAddrVer::V4 => println!("V4"),
        IpAddrVer::V6 => println!("V6"),
    }
}
