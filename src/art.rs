//! ASCII art for the moon, in 16 frames spanning one synodic cycle.
//!
//! Frame `i` depicts the moon at cycle position `i / 16`: frame 0 is new,
//! frame 8 is full, and the shaded side flips from left to right as the moon
//! turns from waxing to waning. The odd-numbered frames are the in-between
//! steps that the eight named phases skip over.

/// Number of distinct frames in one lunar cycle.
pub const FRAME_COUNT: usize = 16;

/// Column width of every frame.
pub const FRAME_WIDTH: usize = 15;

/// The column the moon's disc is centred on.
///
/// The art is padded on the left, so the disc sits right of the middle of
/// `FRAME_WIDTH`. Captions are centred on this column, not on the frame.
pub const FRAME_CENTRE: usize = 9;

#[rustfmt::skip]
const FRAMES: [&str; FRAME_COUNT] = [
    // 0 -- new
"       _..._
     .:::::::.
    :::::::::::
    :::::::::::
    `:::::::::'
      `':::''",
    // 1
"       _..._
     .::::::'.
    ::::::::: :
    ::::::::: :
    `:::::::'.'
      `':::''",
    // 2 -- waxing crescent
"       _..._
     .::::. `.
    :::::::.  :
    ::::::::  :
    `::::::' .'
      `'::'-'",
    // 3
"       _..._
     .::::. `.
    :::::::   :
    :::::::   :
    `:::::'  .'
      `':::''",
    // 4 -- first quarter
"       _..._
     .::::  `.
    ::::::    :
    ::::::    :
    `:::::   .'
      `'::.-'",
    // 5
"       _..._
     .:::'  `.
    ::::'     :
    ::::'     :
    `::::    .'
      `'::.-'",
    // 6 -- waxing gibbous
"       _..._
     .::'   `.
    :::       :
    :::       :
    `::.     .'
      `':..-'",
    // 7
"       _..._
     .:'    `.
    ::        :
    ::        :
    `:.      .'
      `-...-'",
    // 8 -- full
"       _..._
     .'     `.
    :         :
    :         :
    `.       .'
      `-...-'",
    // 9
"       _..._
     .'    `:.
    :       `::
    :        ;:
    `.       :'
      `-...:'",
    // 10 -- waning gibbous
"       _..._
     .'  `:::.
    :      ::::
    :      ::::
    `.     :::'
      `-..:''",
    // 11
"       _..._
     .'  `:::.
    :     :::::
    :     :::::
    `.    ;:::'
      `-.::''",
    // 12 -- last quarter
"       _..._
     .'  ::::.
    :    ::::::
    :    ::::::
    `.   :::::'
      `-.::''",
    // 13
"       _..._
     .' .::::.
    :   :::::::
    :   :::::::
    `.  ':::::'
      `-.::''",
    // 14 -- waning crescent
"       _..._
     .' .::::.
    :  ::::::::
    :  ::::::::
    `. '::::::'
      `-.::''",
    // 15
"       _..._
     .'::::::.
    : :::::::::
    : :::::::::
    `. `::::::'
      `':::''",
];

/// Return the frame that best depicts the given position in the lunar cycle.
///
/// `cycle_fraction` is the elapsed portion of the synodic month, where 0.0 is
/// new moon and 0.5 is full moon. Values outside `[0, 1)` are wrapped, so
/// callers do not have to normalise first.
pub fn frame(cycle_fraction: f64) -> &'static str {
    let wrapped = cycle_fraction.rem_euclid(1.0);
    let index = (wrapped * FRAME_COUNT as f64).round() as usize % FRAME_COUNT;
    FRAMES[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_frame_is_the_same_size() {
        for (i, art) in FRAMES.iter().enumerate() {
            let lines: Vec<&str> = art.lines().collect();
            assert_eq!(lines.len(), 6, "frame {i} has the wrong line count");
            for line in lines {
                assert!(
                    line.chars().count() <= FRAME_WIDTH,
                    "frame {i} is wider than FRAME_WIDTH"
                );
            }
        }
    }

    #[test]
    fn the_disc_is_centred_on_frame_centre() {
        for (i, art) in FRAMES.iter().enumerate() {
            let widest = art
                .lines()
                .max_by_key(|line| line.trim_end().chars().count() - leading_spaces(line))
                .expect("frames are never empty");
            let start = leading_spaces(widest);
            let end = widest.trim_end().chars().count();
            assert_eq!(
                (start + end) / 2,
                FRAME_CENTRE,
                "frame {i} is not centred on FRAME_CENTRE"
            );
        }
    }

    fn leading_spaces(line: &str) -> usize {
        line.chars().take_while(|c| *c == ' ').count()
    }

    #[test]
    fn landmark_phases_map_to_landmark_frames() {
        assert_eq!(frame(0.0), FRAMES[0], "new moon");
        assert_eq!(frame(0.25), FRAMES[4], "first quarter");
        assert_eq!(frame(0.5), FRAMES[8], "full moon");
        assert_eq!(frame(0.75), FRAMES[12], "last quarter");
    }

    #[test]
    fn fractions_outside_the_unit_range_wrap_onto_new_moon() {
        assert_eq!(frame(1.0), FRAMES[0]);
        assert_eq!(frame(2.0), FRAMES[0]);
        assert_eq!(frame(-1.0), FRAMES[0]);
    }

    #[test]
    fn the_end_of_the_cycle_wraps_instead_of_panicking() {
        assert_eq!(frame(0.999), FRAMES[0]);
    }
}
