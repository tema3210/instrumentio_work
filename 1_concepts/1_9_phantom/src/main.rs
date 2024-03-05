use std::marker::PhantomData;

struct Fact<T> {
    _ph: PhantomData<T>,
}

impl<T> Fact<T> {
    fn new() -> Self {
        Self { _ph: PhantomData }
    }
}

static FACTS_FOR_VEC: &[&str] = &[
    "Vec is heap-allocated",
    "Vec may re-allocate on growing",
    "Vec is best collection EVAR!!!!1!",
];

impl Fact<Vec<u64>> {
    fn fact(&self) -> &'static str {
        // stalo bolshe texta =\
        FACTS_FOR_VEC[rand::Rng::gen_range(&mut rand::thread_rng(), 0..FACTS_FOR_VEC.len())]
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
