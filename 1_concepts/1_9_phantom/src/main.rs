use std::{marker::PhantomData, ops::RangeBounds};


struct Fact<T> {
    _ph: PhantomData<T>
}

impl<T> Fact<T> {
    fn new() -> Self {
        Self {
            _ph: PhantomData
        }
    }
}

impl Fact<Vec<u64>> {
    fn fact(&self) -> &'static str {
        let r = rand::random::<usize>();
        let facts = [
            "Vec is heap-allocated",
            "Vec may re-allocate on growing",
            "Vec is best collection EVAR!!!!1!"
        ];
        facts[r % facts.len()]
    }
}


fn main() {
    let f: Fact<Vec<u64>> = Fact::new();
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());
    println!("Fact about Vec: {}", f.fact());
}
