fn main() {
    learn_variables();
    learn_control_flow();
}

fn learn_variables() {
    println!("learn_variables");
    let mut x = 5;
    println!("The x is {}", x);
    x = 6;
    println!("The y is {}", x);

    shadow();
    anther_fun(32);
    give_me_five();
    let x = 32;
    let y = add_one(x);
    println!("Now x is: {}, y is: {}", x, y);
}

/**
 * 变量可以shadwed之前的同名变量
 * 但如果出了作用域，又会回退到之前的变量值
 */
fn shadow() {
    let z = 1;

    let z = z * 2;

    {
        let z = z * 2;
        println!("now Z is {}", z);
    }
    println!("An end, Z is {}", z);
}

/**
* 在Rust中，函数与变量命名遵循蛇形命名规范
* 1. 函数语句没有返回值，如： let x = 3;
* 2. 表达式有返回值如下面函数所示
*   {
*        let x = 3;
*        x * 3
*    }
* 这就是一个表达式，注意 x*3 后面没有分号
*
*/
fn anther_fun(x: i32) {
    println!("Hello World!, {}", x);

    let y = {
        let x = 3;
        x * 3
    };

    println!("now y is: {}", y);
}

/**
 * 函数的返回值，在签名中使用 -> 表示
 */
fn give_me_five() -> i32 {
    5
}

fn add_one(x: u32) -> u32 {
    x + 1
}

fn learn_control_flow() {
    println!("learn_control_flow");
    print_count(0);
    print_count(1);
    show_if_case();
    show_loop_lable();
}

fn print_count(x: i32) {
    if x == 0 {
        println!("No count")
    } else {
        println!("Count is {}", x);
    }
}

fn show_if_case() {
    let contral = false;
    let x = if contral { 1 } else { 3 };
    println!("x is {}", x);
}

fn show_loop_lable() {
    let mut count = 1;
    'counting_up: loop {
        println!("count is {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining is: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}
