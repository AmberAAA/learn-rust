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

fn route( ip_type: IpAddKind ) {}

#[derive(Debug)]
enum IpAddKindWithValue {
    V4(u8, u8, u8, u8),
    V6(String)
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
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

/**
 * 你还可以为枚举自定义方法
 */
impl Message {
    fn call(&self) {
        println!("Type is {:?}", self)
    }
}

enum Option<T> {
    Some(T),
    None
}

fn main() {
    println!("Hello, world!1");

    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1")
    };

    let v4  = IpAddKindWithValue::V4(1,1,1,1);
    let v6 = IpAddKindWithValue::V6(String::from("::1"));

    let v6Struct = IpStruct::V4(IpV4Struct {});

    route(home.kind);

    println!("IP: {:?}", home.address);

    println!("V4: {:?}", v4);

    let message = Message::Write(String::from("Hello World! I'm Amber"));

    message.call();

    let some_number = Some(4);

    
}
