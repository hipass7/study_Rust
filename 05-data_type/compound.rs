fn main()
{
    // array
    let arr: [i32; 3] = [1, 2, 3];
 
    println!("{}", arr[0]); // output : 1st data
    println!("{:?}", arr); // output : entire array (included [])

    // tuple
    let dat: (i32, char, bool) = (1, 'A', true);
    let usr = ("Tom", 'B');

    let a = dat.0;
    let b: char = dat.1;
    let c: bool = dat.2;

    // destructuring
    let (a, b, c) = dat;
    println!("{:?}", dat);
}