use rand::distributions::{Sample, Range};
use self::super::super::util::distance;
use arraydeque::ArrayDeque;
use rand::StdRng;
use quickersort;
use std::u8;


pub struct GenerationContext {
    past_positions: ArrayDeque<[(usize, usize); 4 + 1]>,
    generator: StdRng,
    corners: (Range<usize>, [(usize, usize); 4]),
}

impl GenerationContext {
    pub fn new(mandala_size: (usize, usize)) -> GenerationContext {
        let mut past = ArrayDeque::new();
        for _ in 0..past.capacity() {
            past.push_back((mandala_size.0 / 2, mandala_size.1 / 2));
        }

        GenerationContext {
            past_positions: past,
            generator: StdRng::new().unwrap(),
            corners: (Range::new(0, 4), [(0, 0), (mandala_size.0, 0), (0, mandala_size.1), (mandala_size.0, mandala_size.1)]),
        }
    }

    pub fn gen(&mut self) -> ((u32, u32), [u8; 3]) {
        static MAX_COLOUR: f64 = u8::MAX as f64;

        let &(prev_x, prev_y) = self.past_positions.back().unwrap();
        let (to_x, to_y) = self.corners.1[self.corners.0.sample(&mut self.generator)];

        let x = (prev_x as isize - ((prev_x as isize - to_x as isize) / 2)) as usize;
        let y = (prev_y as isize - ((prev_y as isize - to_y as isize) / 2)) as usize;

        let mut distances = [distance(self.past_positions[0], self.past_positions[1]),
                             distance(self.past_positions[1], self.past_positions[2]),
                             distance(self.past_positions[2], self.past_positions[3]),
                             distance(self.past_positions[3], (x, y))];
        quickersort::sort_floats(&mut distances);

        // TODO: use something actually circular that will take care of looping for us so as to remove the pop_front() call
        self.past_positions.pop_front();
        self.past_positions.push_back((x, y));
        ((x as u32, y as u32),
         [(MAX_COLOUR - ((distances[0] / distances[3]) * MAX_COLOUR).round()).abs() as u8,
          (MAX_COLOUR - ((distances[1] / distances[3]) * MAX_COLOUR).round()).abs() as u8,
          (MAX_COLOUR - ((distances[2] / distances[3]) * MAX_COLOUR).round()).abs() as u8])
    }
}
