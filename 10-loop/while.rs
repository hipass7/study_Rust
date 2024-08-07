fn main() {
    let mut sum = 0;
    let mut i = 1;
 
    while i <= 100 {
        sum += i;        
        i = i + 1;
    }
 
    println!("Sum of 1~100: {}", sum);
}