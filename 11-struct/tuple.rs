// Tuple Struct
struct Color(u8, u8, u8, u8);
 
fn main() {
    // initialize
    let red = Color(255, 0, 0, 3);
 
    // using field
    println!("R={},G={},B={},D={}", red.0, red.1, red.2, red.3);
}