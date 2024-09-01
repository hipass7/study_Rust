struct Person {
    id: i32,
    name: String,
    active: bool
}
 
impl Person {
    fn new(id: i32, name: String) -> Person {
        Person{ id: id, name: name, active: true }
    }
 
    // method (void type)
    fn display(&self) { // fn display(self: &Self) {
        if self.active {
            println!("{}: {}", self.id, self.name);
        }
        else {
            println!("{}: inactive", self.id);
        }
    }
}
 
fn main() {
    let p = Person::new(101, String::from("Tom"));
    p.display();
}