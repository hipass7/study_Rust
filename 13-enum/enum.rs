#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High
}
 
fn main() {
    let urgent = Priority::High;
    dbg!(urgent);
}