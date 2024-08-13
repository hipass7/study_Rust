// declare struct
struct Member {
    fname: String,
    lname: String,
    age: u16,
    active: bool
}
 
fn main() {
    // initialize
    let mem1 = Member {
        active: true,
        fname: String::from("Tom"),
        lname: String::from("Lee"),
        age: 35
    };
 
    // 구조체 필드 읽기
    println!("{}: {}", mem1.fname, mem1.active);
}