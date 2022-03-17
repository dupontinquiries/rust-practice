#[derive(Debug)] // allows for using println!("{:?}", ...) macro
pub struct Person {
    pub name: String,
}

pub fn hello_other_world() {
    println!("hello from another file");
}
