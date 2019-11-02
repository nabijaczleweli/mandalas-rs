extern crate fixed_vec_deque;
extern crate quickersort;
#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate num_cpus;
#[cfg(target_os = "windows")]
extern crate winapi;
extern crate regex;
extern crate image;
#[macro_use]
extern crate clap;
#[cfg(not(target_os = "windows"))]
extern crate libc;
extern crate rand;

mod options;

pub mod ops;
pub mod util;

pub use options::Options;
