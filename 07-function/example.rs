use std::ops::Mul;

fn add(lhs: usize, rhs: usize) -> usize {
    lhs.overflowing_add(rhs).0
}

fn sub(lhs: usize, rhs: usize) -> usize {
    lhs.overflowing_sub(rhs).0
}

struct Mat2 {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl Mat2 {
    fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        }
    }
}

impl Mat2 {
    fn power(self, power: u64) -> Mat2 {
        let mut ret: Mat2 = self;
        let mut idx: u64 = power;
        while idx > 1 {
            ret = ret * self;
            idx -= 1;
        }
        ret
    }
}

struct Vec2 {
    a: u64,
    b: u64,
}

impl Vec2 {
    fn get_upper(self) -> u64 {
        if self.a >= self.b {
            self.a
        }
        else {
            self.b
        }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            a: self.a * rhs.a + self.b * rhs.b,
            b: self.c * rhs.a + self.d * rhs.b,
        }
    }
}

fn main()
{
    println!("{}", add(7, 3));
    println!("{}", sub(7, 3));

    const FIBONACCI_MAT: Mat2 = Mat2 {
        a: 2,
        b: 3,
        c: 4,
        d: 0,
    };

    const FIBONACCI_VEC: Vec2 = Vec2 { a: 1, b: 0 };

    println!("{}", (FIBONACCI_MAT.power(3) * FIBONACCI_VEC).get_upper());

}