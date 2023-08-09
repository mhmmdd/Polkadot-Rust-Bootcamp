// 3. iterator trait
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {

    // 1. iterator
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next()); // None

    // 2. for loop
    let numbers = vec![1, 2, 3, 4, 5];
    for i in numbers.iter() {
        println!("{:?}", i);
    }

    // 3. iterator trait
    let mut fibonacci = Fibonacci { curr: 0, next: 1 };

    println!("Fibonacci sequence");
    for i in fibonacci.take(10) {
        println!("{:?}", i);
    }

    println!("Fibonacci sequence");
    let mut fibonacci = Fibonacci { curr: 0, next: 1 };
    for _ in 0..10 {
        println!("{}", fibonacci.next().unwrap());
    }
}
