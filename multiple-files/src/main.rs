use crate::person::Person;

mod person;

fn main() {
    person::hello_other_world();
    let me = Person {
        name: "Kessler".to_string(),
    };
    println!("{:?}", me);
}
