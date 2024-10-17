use std::env;

use eyre::Result;
use lsdj::{fs::File, sram::SRam};

fn main() -> Result<()> {
    let sram = SRam::from_path(env::args().nth(1).unwrap())?;

    // Get the file with the most saves, probably the most insteresting
    let file = sram
        .filesystem
        .files()
        .max_by(|s1, s2| s1.version().cmp(&s2.version()))
        .unwrap();

    let song = file.lsdsng()?;
    let name = song.name()?;

    let save_count = song.version();

    let song = song.decompress()?;

    println!("{name}:{save_count}");

    for instrument in song.instruments() {
        let instrument = match instrument {
            Ok(i) => i,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        println!(
            "{:02X} {:<5} {:X?}",
            instrument.index(),
            instrument.name(),
            instrument.kind()
        );
    }

    Ok(())
}
