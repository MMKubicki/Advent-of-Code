use common::intcode::{create_computer, ComputerState, Level};
use std::{fs, io, path};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "day02", about = "Day 02 of Advent of Code 2019")]
struct Settings {
    #[structopt(parse(from_os_str))]
    /// Path to file containing the initial memory
    input: path::PathBuf,
}

fn main() -> io::Result<()> {
    let settings = Settings::from_args();

    let mut memory = get_initial_memory_from_file(settings.input)?;
    memory[1] = 12;
    memory[2] = 2;

    let mut computer = create_computer(Level::Day02, memory);

    if computer.run_until_end() == ComputerState::Finished {
        println!(
            "Computer finished with: {}",
            computer
                .get(0)
                .expect("Could not get output memory of computer")
        );
    } else {
        println!("Computer threw error");
    }

    Ok(())
}

fn get_initial_memory_from_file<P: AsRef<path::Path>>(file: P) -> io::Result<Vec<usize>> {
    let res = fs::read_to_string(file)?
        .split(',')
        .map(str::parse::<usize>)
        .filter_map(Result::ok)
        .collect();
    Ok(res)
}
