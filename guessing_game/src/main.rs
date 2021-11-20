/**
 * 自己实现一个猜数字游戏
 */
use std::io;
// Rng 是一个 trait 具体不清楚 后面会学到
use rand::Rng;
// 是一个枚举
use std::cmp::Ordering;

fn main() {
    /*
     * gen_range IDE不会语法提示，是由`use rand::Rng;`带进来的方法。
     * 1..101 是一个范围表达式，表示包含[1, 101)的整之间的数，与 1..=100等价
     */
    let secret_number = rand::thread_rng().gen_range(1..101);

    /*
     * 1. println后面的!表示println是一个宏而不是函数，具体后面再学。
     * 2. {} 是字符串中的占位符
     */
    println!("The secret number is: {}", secret_number);

    // loop 表示一个无限循环
    loop {
        // 构造一个可变的字符串空对象
        let mut guess_number = String::new();

        println!("Please input a value: ");

        /*
         * 解析用户输入，对于可变变量的取引用应该这么写：
         * `&mut guess_number`
         * readline会抛出一个枚举：Resoult，如果OK则赋值，
         * 如果异常则结束异常。并打印expect中的文字。
         * io::stdin()
         *   .read_line(&mut guess_number)
         *   .expect("cant read the number");
         * 其实也可以使用match去实现。
         * macth方法则会去处理枚举
         */

        io::stdin()
            .read_line(&mut guess_number)
            .expect("Cann't handle the value input!");

        /*
         * 这里需要重新应以一下 guess_number的类型。
         * 值得注意的是，rust允许创建多个同名变量，去覆盖上一个的值，一般用在类型转换。
         * trim是为了去除尾部带入的回车
         * 当出现Err的时候直接跳出进入下一个选项
         */
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Cann't parse the value");
                continue;
            }
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }
}
