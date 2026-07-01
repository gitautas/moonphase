use crate::phases::MoonPhase;
use eluna;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

mod phases;

fn main() {
    let mut print_name = false;
    let mut print_percentage = false;

    let args: Vec<String> = env::args().collect();
    args.iter().for_each(|a| {
        if a == "-h" || a == "--help" {
            println!("Print an ascii of the current moon phase");
            println!("");
            println!("");
            println!("Usage: moonphase [-h --help] [-n --name] [-p --percentage]");
            println!("");
            println!("Options:");
            println!("   -h, --help          Print this help message");
            println!("   -n, --name          Display the name of the current moon phase");
            println!("   -p, --percentage    Show how much of the moon is visible tonight");
            println!("");
            println!("   --print_all         Print all icons and exit");
            std::process::exit(0);
        }

        if a == "--print_all" {
            print_all();
            std::process::exit(0);
        }

        if a == "-n" || a == "--name" {
            print_name = true
        }
        if a == "-p" || a == "--percentage" {
            print_percentage = true
        }
    });

    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() as i64,
        Err(_) => panic!("System time is set before the UNIX epoch"),
    };

    let phase = MoonPhase::from(eluna::numeric_phase(now as i64));

    if print_name {
        let indent = 10 - phase.to_string().len() / 2;
        for _ in 0..indent {
            print!(" ")
        }
        println!("{}", phase.to_string())
    }
    println!("{}", phase.ascii());

    let percentage = eluna::fraction(now as i64) * 100.0;
    if print_percentage {
        println!("{}% of the moon is visible tonight", percentage.floor());
    }
}

fn print_all() {
    let blocks: Vec<Vec<String>> = (0..9)
        .map(|i| {
            let p = MoonPhase::from(i);
            let name = p.to_string();
            let indent = " ".repeat(10usize.saturating_sub(name.len() / 2));

            format!("{indent}{name}\n{}", p.ascii())
                .lines()
                .map(String::from)
                .collect()
        })
        .collect();

    let h = blocks.iter().map(Vec::len).max().unwrap_or(0);
    let result: String = (0..h)
        .map(|row| {
            let line: String = blocks
                .iter()
                .map(|b| format!("{:<18}  ", b.get(row).map(String::as_str).unwrap_or("")))
                .collect();
            format!("{}\n", line.get(5..).unwrap_or(""))
        })
        .collect();

    println!("{}", result);
}
