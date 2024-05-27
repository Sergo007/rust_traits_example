#![allow(dead_code)]
trait Tr1 {
    fn get_value(&self) -> Option<i32> {
        println!("Was called get_value from Tr1");
        None
    }
    // fn get_value(&self) -> Option<i32>;
}
fn print_value(s: &dyn Tr1) {
    match s.get_value() {
        Some(v) => println!("Value is {}", v),
        None => println!("Value is not available"),
    }
}
#[derive(Debug)]
struct St1 {
    value: i32,
}
#[derive(Debug)]
struct St2 {
    value: i32,
    value2: i32,
}
#[derive(Debug)]
struct St3 {}
impl Tr1 for St1 {}
impl Tr1 for St2 {
    fn get_value(&self) -> Option<i32> {
        println!("Was called get_value from St2");
        Some(self.value)
    }
}
impl Tr1 for St3 {
    fn get_value(&self) -> Option<i32> {
        println!("Was called get_value from St3");
        Some(100)
    }
}
fn main() {
    let s1 = St1 { value: 10 };
    println!("created  {:?}", s1);
    let s2 = St2 {
        value: 20,
        value2: 30,
    };
    println!("created  {:?}", s2);
    let s3 = St3 {};
    println!("created  {:?}", s3);
    println!("called print_value(&s1)");
    print_value(&s1);
    println!("called print_value(&s2)");
    print_value(&s2);
    println!("called print_value(&s3)");
    print_value(&s3);
}
