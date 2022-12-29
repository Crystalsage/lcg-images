// A state for the generator.
struct LCG {
    X: Vec<u32>,
    Y: Vec<u32>,
    c: u32,
    m: u32,
}

impl LCG {
    // Initialize a generator with a seed
    fn init(x0: u32, c: u32, m: u32) -> Self {
        LCG {
            X: vec![x0],
            Y: Vec::new(),
            c,
            m,
        }
    }

    // Yield the next value from the generator.
    fn next(self: Self) {}
}


fn main() {
    println!("Hello, world!");
}
