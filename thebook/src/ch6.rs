#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

impl IpAddrKind {
    fn to_number(&self) -> u32 {
        match self {
            IpAddrKind::V4 => 4,
            IpAddrKind::V6 => 6,
        }
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn new(addr: &str) -> IpAddr {
        IpAddr {
            address: String::from(addr),
            kind: match addr.contains(':') {
                true => IpAddrKind::V6,
                false => IpAddrKind::V4,
            },
        }
    }
}

#[derive(Debug)]
struct IpNext {}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
    VNext(IpNext),
}

pub fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Four = {}", four.to_number());
    println!("Six = {}", six.to_number());

    let loopback4 = IpAddr::new("127.0.0.1");
    let loopback6 = IpAddr::new("::1");

    println!("Loopback on IPv4: {:?}", loopback4);
    println!("Loopback on IPv6: {:?}", loopback6);

    let loopback4 = IpAddr2::V4(127, 0, 0, 1);
    let loopback6 = IpAddr2::V6(String::from("::1"));
    let loopback_next = IpAddr2::VNext(IpNext {});

    println!("Loopback on IPv4: {:?}", loopback4);
    println!("Loopback on IPv6: {:?}", loopback6);
    println!("Loopback on IPv6: {:?}", loopback_next);

    // Option
    let mut op = Some(5);
    println!("Option = {:?}", op);
    op = None;
    println!("Option = {:?}", op);

    // 6.2. - match
    println!("\n## Chapter 6.2 - match\n");

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // and many more
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky Penny");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(UsState::Alabama) => {
                println!("Noice, an Alabama Quarter!");
                25
            }
            Coin::Quarter(state) => {
                println!("Oh, that quarter is from {:?}!", state);
                25
            }
        }
    }

    println!("Penny = {}", value_in_cents(Coin::Penny));
    println!("Nickel = {}", value_in_cents(Coin::Nickel));
    println!("Dime = {}", value_in_cents(Coin::Dime));
    println!("Qrt = {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("Qrt = {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let none: Option<i32> = None;
    println!("{:?}", plus_one(five));
    println!("{:?}", plus_one(None));
    println!("{:?}", five.map(|i| i + 1));
    println!("{:?}", none.map(|i| i + 1));

    // 6.3 if let
    println!("\n## Chapter 6.3 - if let\n");
    let coins: &[Coin] = &[
        Coin::Penny,
        Coin::Dime,
        Coin::Nickel,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
    ];

    let mut count = 0;
    for coin in coins.iter() {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?} - we won't count that!", state);
        } else if let Coin::Penny = coin {
            println!("That's a lucky {:?} - we won't count  that!", coin);
        } else {
            println!("{:?} counted!", coin);
            count += 1;
        }
    }
    println!("Count: {}", count);
}
