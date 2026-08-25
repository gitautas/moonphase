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

/// Which face of the moon the characters draw.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shading {
    /// Characters are the shadow; blank space is the sunlit face.
    Shadow,
    /// Characters are the sunlit face; blank space is the shadow.
    Light,
}

/// Return the frame that best depicts the given position in the lunar cycle.
///
/// `cycle_fraction` is the elapsed portion of the synodic month, where 0.0 is
/// new moon and 0.5 is full moon. Values outside `[0, 1)` are wrapped, so
/// callers do not have to normalise first.
pub fn frame(cycle_fraction: f64, shading: Shading) -> &'static str {
    // Half a cycle apart, the moon's two faces are exactly swapped: the
    // terminator falls in the same place and the sunlit side flips over. So
    // the shadow drawn at `f + 0.5` *is* the sunlit face at `f`, and inverting
    // the art is a half-cycle shift rather than a second table of frames.
    let shifted = match shading {
        Shading::Shadow => cycle_fraction,
        Shading::Light => cycle_fraction + 0.5,
    };

    let wrapped = shifted.rem_euclid(1.0);
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
        assert_eq!(frame(0.0, Shading::Shadow), FRAMES[0], "new moon");
        assert_eq!(frame(0.25, Shading::Shadow), FRAMES[4], "first quarter");
        assert_eq!(frame(0.5, Shading::Shadow), FRAMES[8], "full moon");
        assert_eq!(frame(0.75, Shading::Shadow), FRAMES[12], "last quarter");
    }

    #[test]
    fn fractions_outside_the_unit_range_wrap_onto_new_moon() {
        assert_eq!(frame(1.0, Shading::Shadow), FRAMES[0]);
        assert_eq!(frame(2.0, Shading::Shadow), FRAMES[0]);
        assert_eq!(frame(-1.0, Shading::Shadow), FRAMES[0]);
        assert_eq!(frame(-1.0, Shading::Light), FRAMES[8]);
    }

    #[test]
    fn the_end_of_the_cycle_wraps_instead_of_panicking() {
        assert_eq!(frame(0.999, Shading::Shadow), FRAMES[0]);
        assert_eq!(frame(0.999, Shading::Light), FRAMES[8]);
    }

    #[test]
    fn lit_shading_draws_the_other_face() {
        // A new moon has nothing lit, so inverting it leaves an empty disc --
        // which is the full moon's outline. A full moon inverts to a solid one.
        assert_eq!(frame(0.0, Shading::Light), FRAMES[8], "new moon inverted");
        assert_eq!(frame(0.5, Shading::Light), FRAMES[0], "full moon inverted");
    }

    #[test]
    fn inverting_twice_is_the_original_frame() {
        for step in 0..FRAME_COUNT {
            let f = step as f64 / FRAME_COUNT as f64;
            let once = frame(f, Shading::Light);
            let twice = frame(f + 0.5, Shading::Light);
            assert_eq!(twice, frame(f, Shading::Shadow), "step {step}");
            assert_ne!(once, twice, "step {step} should differ from its inverse");
        }
    }

    #[test]
    fn the_two_shadings_together_cover_the_whole_disc() {
        // Every cell of the disc is drawn by exactly one of the two shadings,
        // give or take the outline glyphs the two renderings share.
        for step in 0..FRAME_COUNT {
            let f = step as f64 / FRAME_COUNT as f64;
            let shadow = shaded_cells(frame(f, Shading::Shadow));
            let light = shaded_cells(frame(f, Shading::Light));
            let overlap = shadow.iter().filter(|c| light.contains(c)).count();
            let covered = shadow.len() + light.len() - overlap;
            assert!(
                (35..=50).contains(&covered),
                "step {step} covers {covered} cells, expected roughly a disc"
            );
        }
    }

    fn shaded_cells(art: &str) -> Vec<(usize, usize)> {
        art.lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices()
                    .filter(|(_, c)| *c == ':')
                    .map(move |(col, _)| (row, col))
            })
            .collect()
    }
}
