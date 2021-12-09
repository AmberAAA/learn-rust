/**
 * 1. 定义一个没有值的枚举
 */
enum IpAddKind {
    V4,
    V6,
}

/**
 * 2. 定义一个有值的枚举
 */
struct IpAddr {
    kind: IpAddKind,
    address: String,
}

fn route(ip_type: IpAddKind) {}

#[derive(Debug)]
enum IpAddKindWithValue {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpV4Struct {}
struct IpV6Struct {}

/**
 * 枚举的值可以自定义类型
 */
enum IpStruct {
    V4(IpV4Struct),
    V6(IpV6Struct),
}

/**
 * 一个枚举可以混合多个类型
 */
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/**
 * 你还可以为枚举自定义方法
 */
impl Message {
    fn call(&self) {
        println!("Type is {:?}", self)
    }
}

// match 流控制
// 最早，通行者三种货币，金、银、同
enum Coin {
    Gold,
    Silver,
    Copper,
}

fn value_if_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Gold => {
            println!("Gold Coin!");
            100
        }
        Coin::Silver => 10,
        Coin::Copper => 1,
    }
}

// 后来三国，魏吴蜀三个分别铸造了不同的铜币，因此不同的铜币也有了不同的价值
#[derive(Debug)]
enum Country {
    WEI,
    WU,
    SHU
}

#[derive(Debug)]
enum CoinV2 {
    Gold,
    Silver,
    Copper(Country)
}

fn value_if_coin_v2 (coin: CoinV2) -> u8 {
    match coin {
        CoinV2::Gold => {
            println!("Gold Coin!");
            100
        }
        CoinV2::Silver => 10,
        CoinV2::Copper(country) => {
            println!("Owww, {:?}'s Copper Coin", country);
            match country {
                Country::SHU => 1,
                Country::WEI => 2,
                Country::WU => 3,
            }
        }
    } 
}

// OPTION

fn plus_one (num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(value) => Some(value+1 )
    }
}


fn main() {
    println!("Hello, world!1");

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let v4 = IpAddKindWithValue::V4(1, 1, 1, 1);
    let v6 = IpAddKindWithValue::V6(String::from("::1"));

    let v6Struct = IpStruct::V4(IpV4Struct {});

    route(home.kind);

    println!("IP: {:?}", home.address);

    println!("V4: {:?}", v4);

    let message = Message::Write(String::from("Hello World! I'm Amber"));

    message.call();

    let quit = Message::Quit;

    quit.call();

    println!("V1: the gold coin value is {}.", value_if_coin(Coin::Gold));

    println!("V2: the gold coin value is {}.", value_if_coin_v2(CoinV2::Gold));
    println!("V2: the WEI copper coin value is {}.", value_if_coin_v2(CoinV2::Copper(Country::WEI)));

    println!("32 plus one is {:?}", plus_one(Some(32)))
}
