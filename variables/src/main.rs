fn main() {
    let mut x = 5;
    println!("The x is {}", x);
    x = 6;
    println!("The y is {}", x);

    shadow();
}

fn shadow() {
    let z = 1;

    let z = z * 2;

    {
        let z = z * 2;
        println!("now Z is {}", z);
    }
    println!("An end, Z is {}", z);
}
