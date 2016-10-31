use arraydeque::ArrayDeque;
use rand::{StdRng, Rng};


#[derive(Clone)]
pub struct GenerationContext {
    bounds: (usize, usize),
    past_positions: ArrayDeque<[(usize, usize); 4]>,
    pos_generator: StdRng,
}

impl GenerationContext {
    pub fn new(mandala_size: (usize, usize)) -> GenerationContext {
        let mut past = ArrayDeque::new();
        while past.len() != past.capacity() {
            past.push_back((0, 0));
        }

        GenerationContext {
            bounds: mandala_size,
            past_positions: past,
            pos_generator: StdRng::new().unwrap(),
        }
    }
}
