extern crate quickersort;
#[macro_use]
extern crate lazy_static;
extern crate arraydeque;
extern crate itertools;
extern crate num_cpus;
extern crate regex;
extern crate image;
#[macro_use]
extern crate clap;
extern crate rand;

mod options;

pub mod ops;
pub mod util;

pub use options::Options;
