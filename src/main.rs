// fn greet_world() {
//     let southern_germany = "Grüß Gott!";
//     let chinese = "世界，你好";
//     let english = "World, hello";
//     let regions = [southern_germany, chinese, english];
//     for region in regions.iter() {
//         println!("{}", &region);
//     }
// }

// fn main() {
//     greet_world();
// }

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

// fn main() {
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";
 
//     let records = penguin_data.lines();
 
//     for (i, record) in records.enumerate() {
//       if i == 0 || record.trim().len() == 0 {
//         continue;
//       }
 
//       // 声明一个 fields 变量，类型是 Vec
//       // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//       // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
//       let fields: Vec<_> = record
//         .split(',')
//         .map(|field| field.trim())
//         .collect();
//       if cfg!(debug_assertions) {
//           // 输出到标准错误输出
//         eprintln!("debug: {:?} -> {:?}",
//                record, fields);
//       }
 
//       let name = fields[0];
//       // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
//       //
//       // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//       //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//       //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
//       //
//       // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//       if let Ok(length) = fields[1].parse::<f32>() {
//           // 输出到标准输出
//           println!("{}, {}cm", name, length);
//       }
//     }
//   }

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

// fn main() {
//   let (a, mut b): (bool,bool) = (true, false);
//   // a = true,不可变; b = false，可变
//   println!("a = {:?}, b = {:?}", a, b);

//   b = true;
//   assert_eq!(a, b);
// }

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

// struct Struct {
//   e: i32
// }

// fn main() {
//   let (a, b, c, d, e);

//   (a, b) = (1, 2);
//   // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
//   [c, .., d, _] = [1, 2, 3, 4, 5];
//   Struct { e, .. } = Struct { e: 5 };

//   assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
// }

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

// const A_SS: u32 = 200;
// fn main() {
//   {
//     println!(">>>>>>>>>>>>>>>>>>>> {}", A_SS);
//     // const A_SS: u32 = 100;
//   }
//   println!(">>>>>>>>>>>>>>>>>>>>++++++++++++++++ {}", A_SS);

//   // const A_SS: u32 = 100;
// } 

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

// fn main() {
//   let xxx = String::from("hello world");
//   let xxx = 10100;
//   println!(">>> {} <<<", xxx);

//   let guess = "42".parse::<i32>().expect("Not a number!");

//   assert_eq!(100u8.saturating_add(1), 101);
//   assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

  
//   // assert_eq!(100u8.saturating_add(1), 100);
// }














// fn main() {
//   let a : u8 = 255;
//   let b = a.wrapping_add(20);
//   println!("{}", b);  // 19


//   // 断言0.1 + 0.2与0.3相等
//   assert!(0.1 + 0.2 == 0.3);
// }
















// fn main() {
//   let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
//   let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

//   println!("abc (f32)");
//   println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//   println!("         0.3: {:x}", (abc.2).to_bits());
//   println!();

//   println!("xyz (f64)");
//   println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//   println!("         0.3: {:x}", (xyz.2).to_bits());
//   println!();

//   assert!(abc.0 + abc.1 == abc.2);
//   assert!(xyz.0 + xyz.1 == xyz.2);
// }











// fn main() {
//   // let x = (-42.0_f32).sqrt();
//   let x = -42.0_f32.sqrt();
//   if x.is_nan() {
//       println!("未定义的数学行为！！！")
//   } else {
//     println!("啊啊啊")
//   }
// }















// fn main() {
//   // 编译器会进行自动推导，给予twenty i32的类型
//   let twenty = 20;
//   // 类型标注
//   let twenty_one: i32 = 21;
//   // 通过类型后缀的方式进行类型标注：22是i32类型
//   let twenty_two = 22i32;

//   // 只有同样类型，才能运算
//   let addition = twenty + twenty_one + twenty_two;
//   println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

//   // 对于较长的数字，可以用_进行分割，提升可读性
//   let one_million: i64 = 1_000_000;
//   println!("{}", one_million.pow(2));

//   // 定义一个f32数组，其中42.0会自动被推导为f32类型
//   let forty_twos = [
//     42.0,
//     42f32,
//     42.0_f32,
//   ];

//   // 打印数组中第一个值，并控制小数位为2位
//   println!("{:.2}", forty_twos[0]);

//   println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
//   let ssss:f64 = twenty + forty_twos[2];
// }









// fn main() {
//   // 无符号8位整数，二进制为00000010
//   let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;

//   // 二进制为00000011
//   let b: u8 = 3;

//   // {:08b}：左高右低输出二进制01，不足8位则高位补0
//   println!("a value is        {:08b}", a);

//   println!("b value is        {:08b}", b);

//   println!("(a & b) value is  {:08b}", a & b);

//   println!("(a | b) value is  {:08b}", a | b);

//   println!("(a ^ b) value is  {:08b}", a ^ b);

//   println!("(!b) value is     {:08b}", !b);

//   println!("(a << b) value is {:08b}", a << b);

//   println!("(a >> b) value is {:08b}", a >> b);

//   let mut a = a;
//   // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
//   a <<= b;
//   println!("(a << b) value is {:08b}", a);

//   for i in 'a'..='z' {
//     println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          {}",i);
// }
// }






























// fn main() {
//   let c = 'z';
//   let z = 'ℤ';
//   let g = '国';
//   let heart_eyed_cat = '😻';

//   println!("{} {} {} {}", c, z, g, heart_eyed_cat);


//   println!("字符'中'占用了>>> {} <<<字节的内存大小",std::mem::size_of_val(&c));
//   println!("字符'中'占用了>>> {} <<<字节的内存大小",std::mem::size_of_val(&z));
//   println!("字符'中'占用了>>> {} <<<字节的内存大小",std::mem::size_of_val(&g));
//   let qqq = println!("字符'中'占用了>>> {} <<<字节的内存大小",std::mem::size_of_val(&heart_eyed_cat));


//   // println!("qqq {}", qqq);
// }

























// fn main() {
//   assert_eq!(ret_unit_type(), ());
//   println!("Hello, world! Marssssssssssssssssssssssssssssssssssssssss");
// }

// fn ret_unit_type() {
//   let x = 1;
//   // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
//   // 类似三元运算符，在Rust里我们可以这样写
//   let y = if x % 2 == 1 {
//       "odd"
//   } else {
//       "even"
//   };
//   // 或者写成一行
//   let z = if x % 2 == 1 { "odd" } else { "even" };
// }

























// fn main() {
//   let x = 5;
//   let y = &x;

//   assert_eq!(5, x);
//   assert_eq!(5, *y);
//   assert_eq!(5, y);
// }





















// fn main() {
//   let s1 = String::from("hello");

//   let len = calculate_length(&s1);

//   println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//   s.len()
// }

























// fn main() {
//   let mut s = String::from("hello");

//   change(&mut s);
// }

// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

























// fn main() {
//   let mut s = String::from("hello");

//   let r1 = &mut s;
//   let r2 = &mut s;

//   println!("{}, {}", r1, r2);

// }





// fn main() {
//   let mut s = String::from("hello");

//   let r1 = &s; // 没问题
//   let r2 = &s; // 没问题

//   println!("{}, and {}", r1, r2);

//   let r3 = &mut s; // 大问题
  
  
//   // println!("{}, {}, and {}", r1, r2, r3);

//   println!("{} <<<<<<<<<<<<<<<<<<<<<<<<<<<<", r3);
  
// }





























// fn main() {
//   let reference_to_nothing = dangle();
//   println!("++++++++++++++>>>>>>>>>>>>>>> {}", reference_to_nothing)
// }

// fn dangle() -> String {
//   let s = String::from("hello");
//   s
//   // &s
// }



























// #![allow(unused_variables)]
// type File = String;

// fn open(f: &mut File) -> bool {
//     true
// }
// fn close(f: &mut File) -> bool {
//     true
// }

// #[allow(dead_code)]
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

// fn main() {
//     let mut f1 = File::from("f1.txt");
//     open(&mut f1);
//     read(&mut f1, &mut vec![]);
//     close(&mut f1);
// }

































// fn main() {
//     let my_name = "Pascal";
//     // greet(my_name);
//     greet(my_name.to_string());


//     let s = "中国人";
//  let a = &s[0..3];
//  println!("{}",a);



//   }
  
//   fn greet(name: String) {
//     println!("Hello, {}!", name);
//   }



































// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear(); // error!

//     println!("the first word is: {}", word);

//     println!("success...................................")
// }
// fn first_word(s: &String) -> &str {
//     &s[..1]
// }






























// fn greet(name: &str) {
//     println!("Hello, {}!", name);
// }

// fn main() {
//     let my_string = String::from("Alice");
//     greet(&my_string); // 可以传 &String
//     greet("Bob");      // 也可以传 &str

//     let c: char = 'a';
//     println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>// {}", std::mem::size_of_val(&c)); // 输出 4，表示占用 4 字节

//     let s: &str = "a";
//     println!("++++++++++++++++++++++++++++ {}", s.len()); // 输出 1，表示 "a" 只占 1 字节


//     let s1 = String::from("hello");
//    let h = s1[0];
// }
















// fn main() {
//     let mut s = String::from("Hello rust!");
//     s.insert(5, ',');
//     println!("插入字符 insert() -> {}", s);
//     s.insert_str(6, " I like");
//     println!("插入字符串 insert_str() -> {}", s);

//     println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");

//     let mut sss = String::from("洪真英");
//     sss.insert_str(3, "--");
//     println!("插入字符串 insert_str() -> {}", sss);





//     let string_replace = String::from("I like rust. Learning rust is my favorite!");
//     let string_replace = "I like rust. Learning rust is my favorite!";
//     let new_string_replace = string_replace.replace("rust", "RUST");
//     dbg!(new_string_replace);





//     let mut string_replace_range = String::from("I like rust!");
//     string_replace_range.replace_range(7..8, "R");
//     print!(">>>>>>>>>>>>>>>>>>>>       ");
//     dbg!(string_replace_range);
//     println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<");
// }





























// fn main() {
//     let mut string_replace_range = String::from("I like rust!");
//     string_replace_range.replace_range(7..8, "R");
//     dbg!(string_replace_range);
// }

























// fn main() {
//     let mut string_pop = String::from("rust pop 中文!");
//     let p1 = string_pop.pop();
//     let p2 = string_pop.pop();
//     dbg!(p1);
//     dbg!(p2);
//     dbg!(string_pop);
// }




























// fn main() {
//     let mut string_remove = String::from("测试remove方法");
//     println!(
//         "string_remove 占 {} 个字节",
//         std::mem::size_of_val(string_remove.as_str())
//     );
//     // 删除第一个汉字
//     string_remove.remove(0);
//     // 下面代码会发生错误
//     // string_remove.remove(1);
//     // 直接删除第二个汉字
//     // string_remove.remove(3);
//     dbg!(string_remove);
// }






























// fn main() {
//     let mut string_truncate = String::from("测试truncate");
//     string_truncate.truncate(4);
//     dbg!(string_truncate);
// }


























// fn main() {
//     let string_append = String::from("hello ");
//     let string_rust = String::from("rust");
//     // &string_rust会自动解引用为&str
//     let result = string_append + &string_rust;
//     let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
//     result += "!!!";

//     println!("连接字符串 + -> {}", result);
// }
































// fn main() {
//     // 通过 \ + 字符的十六进制表示，转义输出一个字符
//     let byte_escape = "I'm writing \x52\x75\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//     // \u 可以输出一个 unicode 字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//     println!(
//         "Unicode character {} (U+211D) is called {}",
//         unicode_codepoint, character_name
//     );

//     // 换行了也会保持之前的字符串格式
//     // 使用\忽略换行符
//     let long_string = "String literals
//                         can span multiple lines.
//              The linebreak and indentation here ->\
//                         <- can be escaped too!";
//     println!("{}", long_string);
// }


























// fn main() {
//     println!("{}", "hello \\x52\\x75\\x73\\x74");
//     let raw_str = r"Escapes don't work here: \x3F \u{211D}";
//     println!("{}", raw_str);

//     // 如果字符串包含双引号，可以在开头和结尾加 #
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);

//     // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
//     let longer_delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", longer_delimiter);






//     for c in "中国人".chars() {
//         println!("{}", c);
//     }


//     for b in "中国人".bytes() {
//         println!("{}", b);
//     }
    
    
// }


























// fn main() {
//     let mut string_remove = String::from("测试remove方法");
//     println!(
//         "string_remove 占 {} 个字节",
//         std::mem::size_of_val(string_remove.as_str())
//     );
//     // 删除第一个汉字
//     string_remove.remove(0);
//     // 下面代码会发生错误
//     // string_remove.remove(1);
//     // 直接删除第二个汉字
//     // string_remove.remove(3);
//     dbg!(string_remove);


//     let ddd = 'S';
//     // let xxx: Char = 'S';
// }





















// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {}", y);
// }


























// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };

//     println!(">>>>>>>>>>>>>>> {}", user1.active);
//     println!("{:#?}", user2);
    
//     // println!("{:#?}", user1);
// }




























// #[derive(Debug)]
//  struct File {
//    name: String,
//    data: Vec<u8>,
//  }

//  fn main() {
//    let f1 = File {
//      name: String::from("f1.txt"),
//      data: Vec::new(),
//    };

//    let f1_name = &f1.name;
//    let f1_length = &f1.data.len();

//    println!("{:?}", f1);
//    println!("{} is {} bytes long", f1_name, f1_length);

//    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>//////////////////////////");

//    let f3 = &f1;
//    println!("{:?}", f3);
//    println!("{} is {} bytes long", f3.name, f3.data.len());
//    println!("{:#?}", f3.data);


//    println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
//    let black = Color(0, 0, 0);
//     let origin = Point(10, 20, 30);

//     println!("{:#?}", black);

//     println!("{:#?}", origin);

//     let subject = AlwaysEqual;
//  }

//  #[derive(Debug)]
//  struct Color(i32, i32, i32);

//  #[derive(Debug)]
// struct Point(i32, i32, i32);


// struct AlwaysEqual;



































// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: "someone@example.com",
//         username: "someusername123",
//         active: true,
//         sign_in_count: 1,
//     };
// }


































// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 502,
//     };

//     // dbg!(&rect1);
//     dbg!(rect1);


//     let x: i8 = 5;
//     // let y: Option<i8> = Some(5);
//     let y = Some(5);

//     // let sum = x + y;
// }







































// fn main() {
//     // let a = [9, 8, 7, 6, 5];

//     // let first = a[0]; // 获取a数组第一个元素
//     // let second = a[1]; // 获取第二个元素


//     let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];

//     let jan = months[0]; // 获取数组第一个元素
//     println!("The first month is: {}", jan);
// }





























// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();
//     // 读取控制台的输出
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!(
//         "The value of the element at index {} is: {}",
//         index, element
//     );
// }
























// fn main() {
//     // // let array = [String::from("rust is good!"),String::from("rust is good!"),String::from("rust is good!")];
//     // let array = [String::from("rust is good!"); 8];
//     // println!("{:#?}", array);

//     let array: [String; 8] = std::array::from_fn(|_i| {
//         println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {} is called", _i);
//         String::from("rust is good!``````````````")
//     });
//     // 详细解释这行代码

//     println!("{:#?}", array);


// }






























// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5];

//     let slice: &[i32] = &a[1..3];

//     assert_eq!(slice, &[2, 3]);
// }



























// fn main() {

//     // let xxx = 0;

//     // 编译器自动推导出one的类型
//     let one             = [1, 2, 3];
//     // 显式类型标注
//     let two: [u8; 3]    = [1, 2, 3];
//     let blank1          = [0; 3];
//     let blank2: [u8; 3] = [0; 3];
  
//     // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
//     let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];
  
//     // 借用arrays的元素用作循环中
//     for a in &arrays {
//       print!("{:?}: ", a);
//       // 将a变成一个迭代器，用于循环
//       // 你也可以直接用for n in a {}来进行循环
//       for n in a.iter() {
//         print!("\t{} + 10 = {}", n, n+10);
//       }
  
//       let mut sum = 0;
//       // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
//       for i in 0..a.len() {
//         sum += a[i];
//       }
//       println!("\t({:?} = {})", a, sum);
//     }
//   }











// fn main() {
//   let a = [10, 20, 30, 40, 50];
//   let mut index = 0;

//   while index < 5 {
//       println!("the value is: {}", a[index]);

//       index = index + 1;
//   }
// }


























// fn main() {
//   let mut counter = 0;

//   let result = loop {
//       counter += 1;

//       if counter == 10 {
//           break counter * 2;
//       }
//   };

//   println!("The result is {}", result);
// }






































// enum IpAddr {
//   Ipv4,
//   Ipv6
// }

// fn main() {
//    let ip1 = IpAddr::Ipv6;
//    let ip_str = match ip1 {
//        IpAddr::Ipv4 => "127.0.0.1",
//        _ => "::1",
//    };

//    println!("{}", ip_str);
// }




































// enum Action {
//   Say(String),
//   MoveTo(i32, i32),
//   ChangeColorRGB(u16, u16, u16),
// }

// fn main() {
//   let actions = [
//       Action::Say("Hello Rust".to_string()),
//       Action::MoveTo(1,2),
//       Action::ChangeColorRGB(255,255,0),
//   ];
//   for action in actions {
//       match action {
//           Action::Say(s) => {
//               println!("{}", s);
//           },
//           Action::MoveTo(x, y) => {
//               println!("point from (0, 0) move to ({}, {})", x, y);
//           },
//           Action::ChangeColorRGB(r, g, _) => {
//               println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
//                   r, g,
//               );
//           }
//       }
//   }
// }


































// enum Direction {
//   East,
//   West,
//   North,
//   South,
// }

// fn main() {
//   let dire = Direction::South;
//   match dire {
//       Direction::East => println!("East"),
//       Direction::North | Direction::South => {
//           println!("South or North");
//       },
//   };
// }




































// #[derive(Debug)]
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         x => println!("x:other direction: {:?}", x),
//     };
// }





















// fn main() {
//   let v = Some(4_u8);
  
//   let result =  match v {
//         Some(3) => println!("three"),
//         Some(4) => println!("FOUR"),
//         _ => (),
//     };


//     println!("result: {:?}", result);




//     let bar = Some(4);
//     assert!(matches!(bar, Some(x) if x > 12));
// }























// fn main() {
//   let age = Some(30);
//   println!("在匹配前，age是{:?}", age);
//   match age {
//       Some(x) =>  println!("匹配出来的age是{}", x),
//       _ => ()
//   }
//   println!("在匹配后，age是{:?}", age);
// }





































// fn plus_one(x: Option<i32>) -> Option<i32> {
//   match x {
//       Some(i) => Some(i + 1),
//       // None => None,
//       None => Some(-999),
//   }
// }

// fn main() {
//   let five = Some(5);
//   let six = plus_one(five);
//   let none = plus_one(None);
//   println!("five is {:?}", five);
//   println!("six is {:?}", six);
//   println!("none is {:?}", none);
// }
















// struct Point {
//   x: i32,
//   y: i32,
// }

// fn main() {
//   let p = Point { x: 0, y: 7 };

//   match p {
//     Point { x, y } => println!("1111111111111111        On neither axis: ({}, {})", x, y),
//       Point { x, y: 0 } => println!("On the x axis at {}", x),
//       Point { x: 0, y } => println!("2222222222 On the y axis at {}", y),
//   }


//   let ((_, _), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });


//   let arr: &[u16] = &[114, 514];

// if let [x, ..] = arr {
//     assert_eq!(x, &114);
// }

// if let &[.., y] = arr {
//     assert_eq!(y, 514);
// }

// let arr: &[u16] = &[];

// assert!(matches!(arr, [..]));
// assert!(!matches!(arr, [x, ..]));

//   println!("Hello, world!>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>................." ) ;
// }






































// fn main() {
//     enum Message {
//       Hello { id: i32 },
//   }

//   let msg = Message::Hello { id: 3 };

//   match msg {
//     Message::Hello { id: id_variable @ 3..=7 } => {
//         println!("1111111111111Found an id in range: {}", id_variable)
//     },
//     Message::Hello { id: 10..=12 } => {
//         println!("2222222222222Found an id in another range")
//     },
//     Message::Hello { id } => {
//         println!("333333333333Found some other id: {}", id)
//     },
// }

// }

































// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     // 绑定新变量 `p`，同时对 `Point` 进行解构
//     let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
//     println!("x: {}, y: {}", px, py);
//     println!("{:?}", p);


//     let point = Point {x: 10, y: 5};
//     if let p @ Point {x: 10, y} = point {
//         println!("x is 10 and y is {} in {:?}", y, p);
//     } else {
//         println!("x was not 10 :(");
//     }
// }
































// fn main() {
//   match 1 {
//       num @ (1 | 2) => {
//           println!("{}", num);
//       }
//       _ => {}
//   }
// }

































// fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
//   let mut largest = list[0];

//   for &item in list.iter() {
//       if item > largest {
//           largest = item;
//       }
//   }

//   largest
// }

// fn main() {
//   let number_list = vec![34, 50, 25, 100, 65];

//   let result = largest(&number_list);
//   println!("The largest number is {}", result);

//   let char_list = vec!['y', 'm', 'a', 'q'];

//   let result = largest(&char_list);
//   println!("The largest char is {}", result);
// }





























// use std::fmt::Display;

// fn create_and_print<T>() where T: From<i32> + Display {
//     let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
//     println!("a is: {}", a);
// }

// fn main() {
//     create_and_print::<i32>();
// }






























// struct Point<T, U> {
//   x: T,
//   y: U,
// }

// impl<A, S> Point<A, S> {
//   fn mixup<V, W>(self, other: Point<V, W>) -> Point<A, W> {
//       Point {
//           x: self.x,
//           y: other.y,
//       }
//   }
// }

// fn main() {
//   let p1 = Point { x: 5, y: 10.4 };
//   let p2 = Point { x: "Hello", y: 'c'};

//   let p3 = p1.mixup(p2);

//   println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }


































// // 目前只能在nightly版本下使用
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn something<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
//     //       ^-----------------------------^ 这里是一个 const 表达式，换成其它的 const 表达式也可以
// {
//     //
// }

// fn main() {
//     something([0u8; 0]); // ok
//     something([0u8; 512]); // ok
//     something([0u8; 1024]); // 编译错误，数组长度是1024字节，超过了768字节的参数长度限制
// }

// // ---

// pub enum Assert<const CHECK: bool> {
//     //
// }

// pub trait IsTrue {
//     //
// }

// impl IsTrue for Assert<true> {
//     //
// }



































// fn largest<T: PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("111 The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("222 The largest char is {}", result);
// }

























// // use std::convert::TryInto;

// fn main() {
//   let a: i32 = 10;
//   let b: u16 = 100;

//   let b_ = b.try_into()
//             .unwrap();

//   if a < b_ {
//     println!("Ten is less than one hundred.");
//   }
// }




























// use std::ops::Add;

// // 为Point结构体派生Debug特征，用于格式化输出
// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
//     x: T,
//     y: T,
// }

// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;

//     fn add(self, p: Point<T>) -> Point<T> {
//         Point{
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }

// fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
//     a + b
// }

// fn main() {
//     let p1 = Point{x: 1.1f32, y: 1.1f32};
//     let p2 = Point{x: 2.1f32, y: 2.1f32};
//     println!("{:?}", add(p1, p2));

//     let p3 = Point{x: 1i32, y: 1i32};
//     let p4 = Point{x: 2i32, y: 2i32};
//     println!("{:?}", add(p3, p4));
// }





























// #![allow(dead_code)]

// use std::fmt;
// use std::fmt::{Display};

// #[derive(Debug,PartialEq)]
// enum FileState {
//   Open,
//   Closed,
// }

// #[derive(Debug)]
// struct File {
//   name: String,
//   data: Vec<u8>,
//   state: FileState,
// }

// impl Display for FileState {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//      match *self {
//          FileState::Open => write!(f, "OPEN"),
//          FileState::Closed => write!(f, "CLOSED"),
//      }
//    }
// }

// impl Display for File {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "<{} ({})>",
//              self.name, self.state)
//    }
// }

// impl File {
//   fn new(name: &str) -> File {
//     File {
//         name: String::from(name),
//         data: Vec::new(),
//         state: FileState::Closed,
//     }
//   }
// }

// fn main() {
//   let f6 = File::new("f6.txt");
//   //...
//   println!("{:?}", f6);
//   println!("{}", f6);
// }

















































// #[derive(Debug)]
// enum UiObject {
//     Button,
//     SelectBox,
// }

// fn main() {
//     let objects = [
//         UiObject::Button,
//         UiObject::SelectBox
//     ];

//     for o in objects {
//         draw(o)
//     }
// }

// fn draw(o: UiObject) {
//     println!("{:?}",o);
// }


































// trait Draw {
//   fn draw(&self) -> String;
// }

// impl Draw for u8 {
//   fn draw(&self) -> String {
//       format!("u8: {}", *self)
//   }
// }

// impl Draw for f64 {
//   fn draw(&self) -> String {
//       format!("f64: {}", *self)
//   }
// }

// // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
// fn draw1(x: Box<dyn Draw>) {
//   // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
//   x.draw();
// }

// fn draw2(x: &dyn Draw) {
//   x.draw();
// }

// fn main() {
//   let x = 1.1f64;
//   // do_something(&x);
//   let y = 8u8;

//   // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw> 
//   // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
//   draw1(Box::new(x));
//   // 基于 y 的值创建一个 Box<u8> 类型的智能指针
//   draw1(Box::new(y));
//   draw2(&x);
//   draw2(&y);
// }































trait Draw {
  fn draw(&self) -> Self;
}

#[derive(Clone)]
struct Button;
impl Draw for Button {
  fn draw(&self) -> Self {
      return self.clone()
  }
}

fn main() {
  let button = Button;
  let newb = button.draw();
}

pub struct Screen {
  pub components: Vec<Box<dyn Clone>>,
}