//! Option parsing and management.
//!
//! Use the `Options::parse()` function to get the program's configuration,
//! as parsed from the commandline.
//!
//! # Examples
//!
//! ```no_run
//! # use mandalas::Options;
//! let opts = Options::parse();
//! println!("{:#?}", opts);
//! ```


use clap::{self, AppSettings, App, Arg};
use std::env::current_dir;
use std::path::PathBuf;
use std::str::FromStr;
use regex::Regex;
use num_cpus;
use std::cmp;
use std::fs;


lazy_static! {
    static ref RESOLUTION_RGX: Regex = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
}


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The output resolution. Default: `900x900x900`
    pub resolution: (usize, usize, usize),
    /// The directory to put the resulting mandalas in. Default: working directory.
    pub outdir: (String, PathBuf),
    /// The amount of threads to generate points on. Default: 4 or CPU threads - 1.
    pub threads_gen: u64,
    /// The amount of threads to collect on. Default: rest of CPU threads or 1
    pub threads_coll: u64,
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let cpus_total = num_cpus::get();

        let cpus_gen = cmp::max(1, cpus_total * 3 / 4);
        let cpus_coll_s = cmp::max(1, cpus_total - cpus_gen).to_string();

        let cpus_gen_s = cpus_gen.to_string();
        let matches = App::new("mandalas")
            .version(crate_version!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .settings(&[AppSettings::ColoredHelp])
            .args(&[Arg::from_usage("-o --outdir=[OUTPUT_DIR] 'The directory to put the resulting mandalas in. Default: working directory'")
                        .validator(Options::outdir_validator),
                    Arg::from_usage("-s --size 'The output mandala resolution'").default_value("900x900x900").validator(Options::size_validator),
                    Arg::from_usage("-j --jobs-gen 'The amount of threads to use for point generation'")
                        .default_value(&cpus_gen_s)
                        .validator(Options::jobs_validator),
                    Arg::from_usage("-J --jobs-coll 'The amount of threads to use for point collection'")
                        .default_value(&cpus_coll_s)
                        .validator(Options::jobs_validator)])
            .get_matches();

        Options {
            resolution: Options::parse_size(matches.value_of("size").unwrap()).unwrap(),
            outdir: match matches.value_of("outdir") {
                Some(dirs) => (dirs.to_string(), fs::canonicalize(dirs).unwrap()),
                None => {
                    match current_dir() {
                        Ok(mut hd) => {
                            hd = hd.canonicalize().unwrap();

                            fs::create_dir_all(&hd).unwrap();
                            (".".to_string(), hd)
                        }
                        Err(_) => {
                            clap::Error {
                                    message: "Couldn't automatically get current directory, please specify the output directory with the -o option".to_string(),
                                    kind: clap::ErrorKind::MissingRequiredArgument,
                                    info: None,
                                }
                                .exit()
                        }
                    }
                }
            },
            threads_gen: u64::from_str(matches.value_of("jobs-gen").unwrap()).unwrap(),
            threads_coll: u64::from_str(matches.value_of("jobs-coll").unwrap()).unwrap(),
        }
    }

    fn parse_size(s: &str) -> Option<(usize, usize, usize)> {
        RESOLUTION_RGX.captures(s)
            .map(|c| {
                (usize::from_str(c.get(1).unwrap().as_str()).unwrap(),
                 usize::from_str(c.get(2).unwrap().as_str()).unwrap(),
                 usize::from_str(c.get(3).unwrap().as_str()).unwrap())
            })
    }

    fn outdir_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Output directory \"{}\" not found", s))
    }

    fn size_validator(s: String) -> Result<(), String> {
        match Options::parse_size(&s) {
            None => Err(format!("\"{}\" is not a valid size (in format \"NNNxMMM\")", s)),
            Some((0, _, _)) | Some((_, 0, _)) | Some((_, _, 0)) => Err("Can't generate a 0-sized image".to_string()),
            Some(_) => Ok(()),
        }
    }

    fn jobs_validator(s: String) -> Result<(), String> {
        match u64::from_str(&s).ok() {
            None => Err(format!("\"{}\" is not a valid job amount", s)),
            Some(0) => Err("Can't run 0 threads".to_string()),
            Some(_) => Ok(()),
        }
    }
}
