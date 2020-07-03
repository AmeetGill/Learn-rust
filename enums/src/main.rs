enum IpAddrKind1 {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind1,
    address: String,
}

enum IpAddrKind {
    V4(u32,u32,u32,u32),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn main() {
    let home1 = IpAddr {
        kind: IpAddrKind1::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback1 = IpAddr {
        kind: IpAddrKind1::V6,
        address: String::from("::1"),
    };

    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    
    let coin = Coin::Quarter(UsState::Alaska);
    let value_from_fn = value_in_cents(coin);
    
    println!("Valu in cents {}",value_from_fn);
    
    let five: Option<i32> = Some(5);
    let six = plus_one(five);
    
    println!("we got the value {:?} ",six);
    
}
