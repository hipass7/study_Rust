struct Person {
    id: i32,
    name: String,
    active: bool
}
 
impl Person {
    // associated function
    fn new(id: i32, name: String) -> Person {
        Person{ id: id, name: name, active: true }
    }
 
    // method ...
}
 
fn main() {
    // call associated function
    let p = Person::new(101, String::from("Tom"));
 
    println!("{}: {}", p.id, p.name);
}