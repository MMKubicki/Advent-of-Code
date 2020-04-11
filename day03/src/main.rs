use common::wirepanel::Panel;
use std::{fs, io, path};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "day03", about = "Day 03 of Advent of Code 2019")]
struct Settings {
    #[structopt(parse(from_os_str))]
    /// Path to file containing the Panel description
    input: path::PathBuf,
}

fn main() -> io::Result<()> {
    let settings = Settings::from_args();

    let panel = get_panel(settings.input)?;

    let crossed_pts = panel.get_crosses();

    let pt = crossed_pts.get_nearest_by_distance();
    println!("Nearest point by manhatten: {}. Distance: {}", pt.0, pt.1);

    let pt = crossed_pts.get_nearest_by_wire_length();
    println!("Nearest point by wire lenght: {}. Distance: {}", pt.0, pt.1,);

    Ok(())
}

fn get_panel<P: AsRef<path::Path>>(file: P) -> io::Result<Panel> {
    match fs::read_to_string(file)?.parse::<Panel>() {
        Ok(panel) => Ok(panel),
        Err(err) => panic!(err),
    }
}
