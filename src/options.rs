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
use std::fs;


lazy_static! {
    static ref RESOLUTION_RGX: Regex = Regex::new(r"(\d+)x(\d+)").unwrap();
}


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The output resolution. Default: `900x900`
    pub resolution: (usize, usize),
    /// The directory to put the resulting mandalas in. Default: working directory.
    pub outdir: (String, PathBuf),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = App::new("mandalas")
            .settings(&[AppSettings::ColoredHelp])
            .version(crate_version!())
            .author(crate_authors!())
            .about("A cargo subcommand for checking and applying updates to installed executables")
            .args(&[Arg::from_usage("-o --outdir=[OUTPUT_DIR] 'The directory to put the resulting mandalas in. Default: working directory'")
                        .validator(Options::outdir_validator),
                    Arg::from_usage("-s --size 'The output mandala resolution'").default_value("900x900").validator(Options::size_validator)])
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
        }
    }

    fn parse_size(s: &str) -> Option<(usize, usize)> {
        RESOLUTION_RGX.captures(s).map(|c| (usize::from_str(c.at(1).unwrap()).unwrap(), usize::from_str(c.at(2).unwrap()).unwrap()))
    }

    fn outdir_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map(|_| ()).map_err(|_| format!("Output directory \"{}\" not found", s))
    }

    fn size_validator(s: String) -> Result<(), String> {
        match Options::parse_size(&s) {
            None => Err(format!("\"{}\" is not a valid size (in format \"NNNxMMM\")", s)),
            Some((0, _)) | Some((_, 0)) => Err("Can't generate a 0-sized image".to_string()),
            Some(_) => Ok(()),
        }
    }
}
