//! Print an ASCII rendering of the current moon phase.

mod art;
mod phases;

use art::Shading;
use phases::MoonPhase;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

/// Blank columns between frames in `--print-all`.
const GUTTER: usize = 3;

/// How many frames `--print-all` puts on one row before wrapping.
const COLUMNS: usize = 4;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("moonphase: {message}");
            eprintln!("Try 'moonphase --help' for more information.");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        print_help();
        return Ok(());
    }

    if args.contains(["-V", "--version"]) {
        println!("moonphase {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // `--print_all` is the original spelling, kept working for old habits.
    let shading = if args.contains(["-i", "--invert"]) {
        Shading::Light
    } else {
        Shading::Shadow
    };

    if args.contains("--print-all") || args.contains("--print_all") {
        print_all(shading);
        return Ok(());
    }

    let show_name = args.contains(["-n", "--name"]);
    let show_percentage = args.contains(["-p", "--percentage"]);

    let leftovers = args.finish();
    if let Some(unexpected) = leftovers.first() {
        return Err(format!(
            "unrecognised argument '{}'",
            unexpected.to_string_lossy()
        ));
    }

    let now = current_timestamp()?;
    let cycle_fraction = eluna::fraction(now);
    let phase = MoonPhase::from_numeric(eluna::numeric_phase(now))
        .ok_or("eluna reported a moon phase outside the documented 0-8 range")?;

    if show_name {
        println!("{}", centre(phase.name()));
    }

    println!("{}", art::frame(cycle_fraction, shading));

    if show_percentage {
        let lit = phases::illuminated_fraction(cycle_fraction) * 100.0;
        println!("{}% of the moon is visible tonight", lit.round());
    }

    Ok(())
}

/// Seconds since the Unix epoch.
fn current_timestamp() -> Result<i64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .map_err(|_| String::from("the system clock is set before the Unix epoch"))
}

/// Pad `caption` on the left so that it sits centred over the moon's disc.
fn centre(caption: &str) -> String {
    let half = caption.chars().count() / 2;
    let padding = art::FRAME_CENTRE.saturating_sub(half);
    format!("{}{caption}", " ".repeat(padding))
}

/// Print every named phase alongside its art.
fn print_all(shading: Shading) {
    let blocks: Vec<Vec<String>> = MoonPhase::ALL
        .iter()
        .map(|phase| {
            let mut lines = vec![centre(phase.name())];
            let art = art::frame(phase.cycle_fraction(), shading);
            lines.extend(art.lines().map(String::from));
            lines
        })
        .collect();

    // The longest name is wider than the art, so the column has to fit both.
    let column = blocks
        .iter()
        .flatten()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(art::FRAME_WIDTH)
        + GUTTER;

    for row in blocks.chunks(COLUMNS) {
        let height = row.iter().map(Vec::len).max().unwrap_or(0);
        for line_number in 0..height {
            let line: String = row
                .iter()
                .map(|block| {
                    let text = block.get(line_number).map(String::as_str).unwrap_or("");
                    format!("{text:<column$}")
                })
                .collect();
            println!("{}", line.trim_end());
        }
        println!();
    }
}

fn print_help() {
    println!("Print an ascii of the current moon phase");
    println!();
    println!("Usage: moonphase [-n --name] [-p --percentage] [-i --invert]");
    println!("       moonphase [-h --help] [-V --version] [--print-all]");
    println!();
    println!("Options:");
    println!("   -h, --help          Print this help message");
    println!("   -V, --version       Print the version and exit");
    println!("   -n, --name          Display the name of the current moon phase");
    println!("   -p, --percentage    Show how much of the moon is visible tonight");
    println!("   -i, --invert        Draw the lit face instead of the shadow");
    println!();
    println!("   --print-all         Print all icons and exit");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captions_are_centred_over_the_art() {
        assert_eq!(centre("Full Moon"), "     Full Moon");
        assert_eq!(centre("Waxing Crescent"), "  Waxing Crescent");
    }

    #[test]
    fn captions_wider_than_the_art_are_not_indented_or_panicked_on() {
        let long = "A Phase With A Very Long Name";
        assert_eq!(centre(long), long);
    }
}
