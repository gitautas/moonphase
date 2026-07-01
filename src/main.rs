use crate::phases::MoonPhase;
use eluna;
use std::time::{SystemTime, UNIX_EPOCH};

mod phases;

fn main() {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() as i64,
        Err(_) => panic!("System time is set before the UNIX epoch"),
    };

    let phase = MoonPhase::from(eluna::numeric_phase(now as i64));
    let indent = 10 - phase.to_string().len() / 2;
    for _ in 0..indent {
        print!(" ")
    }
    println!("{}", phase.to_string());
    println!("{}", phase.ascii());

    // for i in 0..9 {
    //     let p = MoonPhase::from(i);
    //     let indent = 10 - p.to_string().len() / 2;
    //     for _ in 0..indent {
    //         print!(" ")
    //     }
    //     println!("{}", p.to_string());
    //     println!("{}\n", p.ascii());
    // }
}
