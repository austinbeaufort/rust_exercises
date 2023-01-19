// impl_and_struct

struct Person {
    name: String,
}

impl Person {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, val: &str) {
        self.name = String::from(val);
    }
}

fn main() {
    let mut p1 = Person { name: String::from("Joe") };
    println!("{}", p1.get_name());
    p1.set_name("Frank");
    println!("{}", p1.get_name());
}