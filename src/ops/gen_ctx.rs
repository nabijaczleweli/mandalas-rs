use rand::distributions::{Sample, Range};
use self::super::super::util::distance;
use arraydeque::ArrayDeque;
use rand::StdRng;
use quickersort;
use std::f64;
use std::u8;


/// A generator for mandala points for a canvas of a given size.
pub struct GenerationContext {
    past_positions: ArrayDeque<[(usize, usize, usize); 5 + 1]>,
    generator: StdRng,
    corners: (Range<usize>, Vec<(usize, usize, usize)>),
    dist_div: f64,
}

impl GenerationContext {
    /// Prepare a new, clean, generation context.
    ///
    /// Whole history is initialised to the canvas' center.
    pub fn new(mandala_size: (usize, usize, usize)) -> GenerationContext {
        let mut past = ArrayDeque::new();
        for _ in 0..past.capacity() {
            past.push_back((mandala_size.0 / 2, mandala_size.1 / 2, mandala_size.2 / 2));
        }

        GenerationContext {
            past_positions: past,
            generator: StdRng::new().unwrap(),
            corners: (Range::new(0, 8),
                      vec![(0, 0, 0),
                           (mandala_size.0, 0, 0),
                           (0, mandala_size.1, 0),
                           (mandala_size.0, mandala_size.1, 0),
                           (0, 0, mandala_size.2),
                           (mandala_size.0, 0, mandala_size.2),
                           (0, mandala_size.1, mandala_size.2),
                           (mandala_size.0, mandala_size.1, mandala_size.2)]),
            dist_div: 2f64,
        }
    }

    /// Generate a mandala point.
    ///
    /// The return value is in the format `((pos_x, pos_y, pos_z), [r, g, b])`.
    pub fn gen(&mut self) -> ((u32, u32, u32), [u8; 3]) {
        static MAX_COLOUR: f64 = u8::MAX as f64;

        let &(prev_x, prev_y, prev_z) = self.past_positions.back().unwrap();
        let (to_x, to_y, to_z) = self.corners.1[self.corners.0.sample(&mut self.generator)];

        let x = (prev_x as f64 - ((prev_x as f64 - to_x as f64) / self.dist_div)) as usize;
        let y = (prev_y as f64 - ((prev_y as f64 - to_y as f64) / self.dist_div)) as usize;
        let z = (prev_z as f64 - ((prev_z as f64 - to_z as f64) / self.dist_div)) as usize;

        let distances = [distance(self.past_positions[0], self.past_positions[1]),
                         distance(self.past_positions[1], self.past_positions[2]),
                         distance(self.past_positions[2], self.past_positions[3]),
                         distance(self.past_positions[3], self.past_positions[4]),
                         distance(self.past_positions[4], (x, y, z))];
        let mut distances2 = distances.clone();
        quickersort::sort_floats(&mut distances2);
        let max = distances2[4];
        let mut distances_i = distances.iter().filter(|&d| *d != max).map(|&d| d);
        let distances = [distances_i.next().unwrap_or(distances[0]),
                         distances_i.next().unwrap_or(distances[1]),
                         distances_i.next().unwrap_or(distances[2]),
                         distances_i.next().unwrap_or(distances[3])];

        // TODO: use something actually circular that will take care of looping for us so as to remove the pop_front() call
        self.past_positions.pop_front();
        self.past_positions.push_back((x, y, z));
        ((x as u32, y as u32, z as u32),
         [(MAX_COLOUR - ((distances[0] / max) * MAX_COLOUR).round()).abs() as u8,
          (MAX_COLOUR - ((distances[1] / max) * MAX_COLOUR).round()).abs() as u8,
          (MAX_COLOUR - ((distances[2] / max) * MAX_COLOUR).round()).abs() as u8])
    }
}
