struct Fib {
    a: u128,
    b: u128,
}

impl Iterator for Fib {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(v)
    }
}

fn fib_iter() -> Fib {
    Fib { a: 0, b: 1 }
}

fn main() {
    let x = fib_iter().nth(50).unwrap();
    println!("{}", x);
}

