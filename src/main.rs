use std::{
    env::{args, Args},
    io::{Error, ErrorKind},
};

mod complex;
mod utils;
mod one_dim;
mod two_dim;

fn main() {
    // if the program is going to crash, it should do so here
    let cfg = Config::construct(args()).unwrap();
    if !cfg.vis() {
        println!("Visualization deactivated. Running debug mode.");
    }
    match cfg.dims() {
        1 => one_dim::run(cfg.vis()),
        2 => two_dim::run(cfg.vis()),
        _ => unimplemented!(),
    }
}

#[derive(Debug)]
struct Config {
    dims: u8,
    vis: bool,
}
impl Config {
    pub fn construct(args: Args) -> Result<Self, Error> {
        // shadow iterator without first argument (not needed)
        let mut args = args.skip(1);
        // make sure that the argument length is correct
        if args.len() < 1 || args.len() > 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Incorrect number of arguments. Expected 2 env arguments, found {}. Usage: cargo run --release -- ['number of dimensions'] [Optional 'visible']", args.len()),
            ));
        }

        // note: unwrapping is safe because we've already checked the number of arguments
        let dims = args
            .next()
            .unwrap()
            .parse::<u8>()
            .expect("Failed to parse number of dimensions");
        if dims > 3 || dims < 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Invalid number of dimensions. Expected between 1 and three, got {}",
                    dims
                ),
            ));
        }

        let vis = if let Some(vis_arg) = args.next() {
            if vis_arg != "1" && vis_arg != "true" {
                // if there exists a vis_arg and it is neither 1 nor true, assume false
                false
            } else {
                // both 1 and true are valid arguments for true
                true
            }
        } else {
            // defaiult to true if no value was specified
            true
        };

        Ok(Self { dims, vis })
    }
    pub fn dims(&self) -> u8 {
        self.dims
    }
    pub fn vis(&self) -> bool {
        self.vis
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { dims: 1, vis: true }
    }
}
