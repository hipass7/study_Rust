fn main()
{
    write_header();
    print_data(101, 'F', true);
    let c = add(1, 2);
    println!("{}", c);
}

fn write_header()
{
    println!("Copyright (c) hipass7");
}

fn print_data(id: i16, score: char, retest: bool)
{
    println!("ID: {}, SCORE: {}, RETEST: {}", id, score, retest);
}

fn add(a: i32, b: i32) -> i32
{
    dbg!(a, b); // debugging macro
    return a + b;
}

// 함수 사용법은 Python과 좀 더 비슷해보이는...?
