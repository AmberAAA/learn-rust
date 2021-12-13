pub fn init() {
  println!("Hello, Init!");

  // 创建
  let mut v: Vec<i32> = Vec::new();
  v.push(0);
  v.push(1);
  // 添加
  v.push(2);
  println!("Vector is {:?}", v);

  // 通过宏去创建
  let v2 = vec![1, 2, 3, 4, 5];
  println!("Vector 2 is {:?}", v2);

  // 遍历
  for elem in &v2 {
    println!("Value is: {}", elem);
  };

  // 获取
  match v2.get(3) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  };

  let num = &v2[3];

  println!("{}", num + 1)




  // println!("Value is: {}", )
}
