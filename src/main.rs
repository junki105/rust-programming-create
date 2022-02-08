use std::io::Write;

fn main() {
    println!("Hello, world!");

    // chap1();
    chap2();
}

fn chap2() {
    let i8: i8 = 1;
    let i16: i16 = 1;
    let i32: i32 = 1;
    let i64: i64 = 1;
    let i128: i128 = 1;

    let u8: u8 = 1;
    let u16: u16 = 1;
    let u32: u32 = 1;
    let u64: u64 = 1;
    let u128: u128 = 1;

    // let f8: f8 = 10.1;
    let f32: f32 = 10.1;
    let f64: f64 = 10.1;

    let s1: String = String::from("hello world");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    let mut t = (1, "2");
    t.0 = 1;
    t.1 = "hoge";

    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", &a[1..3]);

    let p = Person {
        name: String::from("John"),
        age: 32,
    };

    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x: 10, y: 20 };

    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err {}", err),
    };

    let result2: Result<i32, String> = Ok(200);

    if let Ok(code) = result2 {
        println!("code: {}", code);
    }

    let result3: Result<i32, String> = Ok(200);

    if let Ok(code) = result3 {
        println!("code: {}", code);
    }

    let result4: Result<i32, String> = Ok(200);
    println!("code: {}", result4.unwrap_or(-1));
    let result5: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result5.unwrap_or(-1));

    let result6: Result<i32, String> = Ok(200);
    let next_result = result6.and_then(func);
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func);

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];
    let v = vec![1, 2, 3, 4, 5];
    println!("{}", v[0]);

    let v = vec![1, 2, 3, 4, 5];
    for el in &v {
        println!("{}", el);
    }

    let byte_array = [b'h', b'e', b'1', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'w', b'o', b'r', b'l', b'd', b'!'];
    print(Box::new(byte_array));

    let immut_val = 10;
    let mut mut_val = 20;
    mut_val += immut_val;

    let v1: u64 = 10;
    let v2 = 10u64;

    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number <= 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }

    let number = 1;
    let result = if 0 <= number {
        number
    } else {
        -number
    };

    let mut count = 0;
    let result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    let mut count = 0;
    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    let count: i32;
    for count in 0..10 {
        println!("count: {}", count);
    }

    let array = [0, 1, 2, 3, 4, 5, 6, 6, 7, 8, 9];
    for el in &array {
        println!("element: {}", el);
    }

    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;
            println!("sub loop end");
        }
        println!("main loop end");
    }

    let i = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        _ => println!("misc"),
    }

    enum Color {
        Red,
        Blue,
        Green,
    }

    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }

    let result: Result<i32, String> = Ok(100);
    // let result: Result<i32, String> = Err("hoge".to_string());
    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        }
    };

    for number in 1..5 {
        println!("{}", number);
    }

    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }

    let x = add(1, 2);
    println!("x = {}", x);

    let p = Person2 {
        name: String::from("Taro"),
        age: 20,
    };

    p.say_name();
    p.say_age();

    let p = Person3 {
        name: String::from("Taro"),
        age: 20,
    };
    p.say_age().say_name();

    let p = Person3::new("taro", 20);
    p.say_name().say_age();

    let s = "Ab23";
    let s = format!("{}-{:?}", s, ("D", 5));
    println!("{}", s);
    let s = format!("{}{}", "abc", "def");
    println!("{}", s);
    let s = concat!("A", "b2", 3);
    println!("{}", s);

    print!("hello");
    println!("hello {}", "world");
    eprint!("hello {}", "error");
    eprintln!("heelo");

    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    writeln!(&mut w, " is 123");
    dbg!(w);

    // panic!("it will panic");

    let v = vec![1, 2, 3];

    println!("defined in file: {}", file!());
    println!("defined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));
}

struct Person3 {
    name: String,
    age: u32,
}

impl Person3 {
    fn say_name(&self) -> &Self {
        println!("I'm {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I'm {} years old.", self.age);
        self
    }

    fn new(name: &str, age: u32) -> Person3 {
        Person3 {
            name: String::from(name),
            age: age,
        }
    }
}

struct Person2 {
    name: String,
    age: u32,
}

impl Person2 {
    fn say_name(&self) {
        println!("I'm {}", self.name);
    }

    fn say_age(&self) {
        println!("I'm {} years old.", self.age);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn abs(num: i32) -> i32 {
    if num < 0 {
        return -num;
    }
    num
}

struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}

fn print(s: Box<[u8]>) {
    println!("{:?}", s);
}

fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    let code = result?;
    println!("code: {}", code);
    Ok(100)
}

fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Ok(100)
}

struct Person {
    name: String,
    age: u32,
}

enum Event {
    Quit,
    KeyDown(u8),
    MouseDown {
        x: i32,
        y: i32,
    },
}

fn chap1() {
    // let mut x = 5;
    //
    // println!("number is: {}", x);
    // x = 6;
    // println!("number is: {}", x);

    // let objectvie: Option<i32> = Some(0);
    // let objectvie: Option<i32> = Some(1);
    // let objectvie: Option<i32> = None;
    // match objectvie {
    //     Some(x) if x % 2 == 0 => println!("even number: {}", x),
    //     Some(x) => println!("odd number: {}", x),
    //     None => println!("no value"),
    // }

    // let mut v = vec![];
    // v.push(1);

    let dog = Dog {};
    let cat = Cat {};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    fn linespan(&self) -> u32;
    fn scientific_name(&self) -> String;
}

struct Dog;

impl Animal for Dog {
    fn linespan(&self) -> u32 {
        13
    }

    fn scientific_name(&self) -> String {
        "Canis lupus familiaris".to_string()
    }
}

struct Cat;

impl Animal for Cat {
    fn linespan(&self) -> u32 {
        16
    }

    fn scientific_name(&self) -> String {
        "Felis catus".to_string()
    }
}

fn show_animal_data<T: Animal>(animal: T) {
    println!("Lifespan: {} years", animal.linespan());
    println!("Scientific name: {}", animal.scientific_name());
}