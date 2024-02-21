fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s)
}

#[derive(Clone, Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Clone> Trinity<T> {
    fn rotate(&mut self) {
        std::mem::swap(&mut self.a, &mut self.b);
        std::mem::swap(&mut self.b, &mut self.c);
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: Clone + PartialEq> Solver<T> {
    fn resolve(&mut self) {
        let mut unsolved = std::mem::take(&mut self.unsolved);

        'l: for t in unsolved.iter_mut() {
            for _ in 0..3 {
                if *t == self.expected {
                    continue 'l;
                }
                t.rotate();
            }
        };
        let _ = std::mem::replace(&mut self.unsolved, unsolved);
    }
}
