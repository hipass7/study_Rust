fn main() {
    let arr = [1,2,3,4,5];
 
    for i in arr {
        println!("{}", i);
    }

    let mut sum = 0;
    for i in 1..101 {
        sum += i;
    }
    println!("Sum of 1~100: {}", sum);

    let mut i = 0;
    let mut sum = 0;

    let result = loop {
        i += 1;
 
        if i % 2 == 1 {
            continue;
        }
 
        sum += i;
 
        if i == 10 {
            break sum / 2
        }
    };

    println!("{}", result);
}