
// following tutorial from https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("i32 {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("f64 {}", self)
    }
}

fn main() {
    let a = Box::new(42 as i32); // box is like a smart pointer
    let b = Box::new(3.14 as f64);
    let c: f64 = (10 * (*a)) as f64 + 0.0001;
    let v: Vec<Box<dyn Show>> = vec![a,b,Box::new(c)]; // a vec of smrt pntrs
    for e in &v {
        println!("show {}", e.show());
    }
}
