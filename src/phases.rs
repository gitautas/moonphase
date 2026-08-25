//! The eight traditional moon phases, and the illumination maths behind them.

use std::f64::consts::TAU;
use std::fmt;

/// One of the eight traditional moon phases, in the order they occur.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MoonPhase {
    NewMoon = 0,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

impl MoonPhase {
    /// Every phase, in the order they occur over a lunar cycle.
    pub const ALL: [MoonPhase; 8] = [
        MoonPhase::NewMoon,
        MoonPhase::WaxingCrescent,
        MoonPhase::FirstQuarter,
        MoonPhase::WaxingGibbous,
        MoonPhase::FullMoon,
        MoonPhase::WaningGibbous,
        MoonPhase::LastQuarter,
        MoonPhase::WaningCrescent,
    ];

    /// Convert one of `eluna`'s numeric phase ids into a phase.
    ///
    /// `eluna` numbers the phases 0-8 rather than 0-7: it splits the new moon
    /// into the sliver at the start of the cycle and the sliver at the end, so
    /// 8 wraps back onto 0. Anything else is out of range.
    pub fn from_numeric(numeric: u8) -> Option<Self> {
        match numeric {
            8 => Some(MoonPhase::NewMoon),
            n => Self::ALL.get(n as usize).copied(),
        }
    }

    /// The human-readable name of the phase.
    pub fn name(&self) -> &'static str {
        match self {
            MoonPhase::NewMoon => "New Moon",
            MoonPhase::WaxingCrescent => "Waxing Crescent",
            MoonPhase::FirstQuarter => "First Quarter",
            MoonPhase::WaxingGibbous => "Waxing Gibbous",
            MoonPhase::FullMoon => "Full Moon",
            MoonPhase::WaningGibbous => "Waning Gibbous",
            MoonPhase::LastQuarter => "Last Quarter",
            MoonPhase::WaningCrescent => "Waning Crescent",
        }
    }

    /// The point in the lunar cycle this phase sits at, in `[0, 1)`.
    ///
    /// The eight phases are evenly spaced, so this is the representative
    /// moment for the phase rather than the whole span it covers.
    pub fn cycle_fraction(&self) -> f64 {
        *self as u8 as f64 / Self::ALL.len() as f64
    }
}

impl fmt::Display for MoonPhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The fraction of the moon's disc that is lit, in `[0, 1]`.
///
/// `cycle_fraction` is the elapsed portion of the synodic month, as returned
/// by [`eluna::fraction`]. Note that the two are *not* the same number: the
/// cycle fraction advances linearly, while illumination follows the cosine of
/// the phase angle, so a moon 42% of the way through its cycle is about 95%
/// lit, not 42%.
pub fn illuminated_fraction(cycle_fraction: f64) -> f64 {
    (1.0 - (TAU * cycle_fraction).cos()) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_ids_cover_the_whole_eluna_range() {
        assert_eq!(MoonPhase::from_numeric(0), Some(MoonPhase::NewMoon));
        assert_eq!(MoonPhase::from_numeric(4), Some(MoonPhase::FullMoon));
        assert_eq!(MoonPhase::from_numeric(7), Some(MoonPhase::WaningCrescent));
        assert_eq!(MoonPhase::from_numeric(8), Some(MoonPhase::NewMoon));
    }

    #[test]
    fn out_of_range_ids_are_rejected_rather_than_panicking() {
        assert_eq!(MoonPhase::from_numeric(9), None);
        assert_eq!(MoonPhase::from_numeric(u8::MAX), None);
    }

    #[test]
    fn phases_are_evenly_spaced_around_the_cycle() {
        assert_eq!(MoonPhase::NewMoon.cycle_fraction(), 0.0);
        assert_eq!(MoonPhase::FirstQuarter.cycle_fraction(), 0.25);
        assert_eq!(MoonPhase::FullMoon.cycle_fraction(), 0.5);
        assert_eq!(MoonPhase::LastQuarter.cycle_fraction(), 0.75);
    }

    #[test]
    fn illumination_peaks_at_full_and_bottoms_out_at_new() {
        assert!(illuminated_fraction(0.0).abs() < 1e-9);
        assert!((illuminated_fraction(0.5) - 1.0).abs() < 1e-9);
        assert!((illuminated_fraction(0.25) - 0.5).abs() < 1e-9);
        assert!((illuminated_fraction(0.75) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn illumination_is_never_outside_zero_to_one() {
        for step in 0..1000 {
            let lit = illuminated_fraction(step as f64 / 1000.0);
            assert!((0.0..=1.0).contains(&lit), "{lit} out of range");
        }
    }

    #[test]
    fn a_gibbous_moon_is_mostly_lit() {
        // The bug this replaced reported the cycle fraction as illumination,
        // so a moon 42% through its cycle was printed as "42% visible".
        let lit = illuminated_fraction(0.4256);
        assert!(lit > 0.9, "expected a nearly-full moon, got {lit}");
    }
}
