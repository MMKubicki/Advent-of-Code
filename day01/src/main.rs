use common::module::*;
use std::{io, path};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "day01", about = "Day 01 of Advent of Code 2019")]
enum Settings {
    /// Use simple calculation to get fuel from mass.
    /// fuel = floor(mass / 3) - 2
    Simple {
        #[structopt(parse(from_os_str))]
        /// Path to file containing the module masses
        input: path::PathBuf,
    },
    /// Use true calculation to get fuel for mass.
    /// Respects the weight of fuel.
    True {
        #[structopt(parse(from_os_str))]
        /// Path to file containing the module masses
        input: path::PathBuf,
    },
}

fn main() -> io::Result<()> {
    let fuel_need = match Settings::from_args() {
        Settings::Simple { input } => {
            do_for_each_mass_in_file_sum(&input, Module::get_simple_fuel_need)?
        }
        Settings::True { input } => {
            do_for_each_mass_in_file_sum(&input, Module::get_true_fuel_need)?
        }
    };

    println!("Needed fuel: {}", fuel_need);

    Ok(())
}

/// Open file, parse each line to Module and apply given function (most likely simple fuel or true fuel calculation)
fn do_for_each_mass_in_file_sum<P: AsRef<path::Path>, F: Fn(Module) -> Fuel>(
    path: P,
    todo: F,
) -> io::Result<Fuel> {
    common::util::do_on_each_line_of_file_sum(path, |str| {
        todo(str.parse::<Module>().unwrap_or_default())
    })
}
