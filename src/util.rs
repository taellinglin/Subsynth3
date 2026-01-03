//! General conversion functions and utilities.

mod stft;
pub mod window;

pub use stft::StftHelper;

pub const MINUS_INFINITY_DB: f32 = -100.0;
pub const MINUS_INFINITY_GAIN: f32 = 1e-5; // 10f32.powf(MINUS_INFINITY_DB / 20)
pub const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Temporarily allow allocations within `func` if NIH-plug was configured with the
/// `assert_process_allocs` feature.
#[cfg(all(debug_assertions, feature = "assert_process_allocs"))]
pub fn permit_alloc<T, F: FnOnce() -> T>(func: F) -> T {
    assert_no_alloc::permit_alloc(func)
}

/// Temporarily allow allocations within `func` if NIH-plug was configured with the
/// `assert_process_allocs` feature.
#[cfg(not(all(debug_assertions, feature = "assert_process_allocs")))]
pub fn permit_alloc<T, F: FnOnce() -> T>(func: F) -> T {
    func()
}

/// Convert decibels to a voltage gain ratio, treating anything below -100 dB as minus infinity.
#[inline]
pub fn db_to_gain(dbs: f32) -> f32 {
    if dbs > MINUS_INFINITY_DB {
        10.0f32.powf(dbs * 0.05)
    } else {
        0.0
    }
}

/// Convert a voltage gain ratio to decibels. Gain ratios that aren't positive will be treated as
/// [`MINUS_INFINITY_DB`].
#[inline]
pub fn gain_to_db(gain: f32) -> f32 {
    f32::max(gain, MINUS_INFINITY_GAIN).log10() * 20.0
}

/// An approximation of [`db_to_gain()`] using `exp()`. Does not treat values below
/// [`MINUS_INFINITY_DB`] as 0.0 gain to avoid branching. As a result this function will thus also
/// never return 0.0 for normal input values. Will run faster on most architectures, but the result
/// may be slightly different.
#[inline]
pub fn db_to_gain_fast(dbs: f32) -> f32 {
    const CONVERSION_FACTOR: f32 = std::f32::consts::LN_10 / 20.0;
    (dbs * CONVERSION_FACTOR).exp()
}

/// [`db_to_gain_fast()`], but this version does truncate values below [`MINUS_INFINITY_DB`] to 0.0.
/// Bikeshedding over a better name is welcome.
#[inline]
pub fn db_to_gain_fast_branching(dbs: f32) -> f32 {
    if dbs > MINUS_INFINITY_DB {
        db_to_gain_fast(dbs)
    } else {
        0.0
    }
}

/// An approximation of [`gain_to_db()`] using `ln()`. Will run faster on most architectures, but
/// the result may be slightly different.
#[inline]
pub fn gain_to_db_fast(gain: f32) -> f32 {
    const CONVERSION_FACTOR: f32 = std::f32::consts::LOG10_E * 20.0;
    f32::max(gain, MINUS_INFINITY_GAIN).ln() * CONVERSION_FACTOR
}

/// [`db_to_gain_fast()`], but the minimum gain value is set to [`f32::EPSILON`]instead of
/// [`MINUS_INFINITY_GAIN`]. Useful in conjunction with [`db_to_gain_fast()`].
#[inline]
pub fn gain_to_db_fast_epsilon(gain: f32) -> f32 {
    const CONVERSION_FACTOR: f32 = std::f32::consts::LOG10_E * 20.0;
    f32::max(gain, MINUS_INFINITY_GAIN).ln() * CONVERSION_FACTOR
}

/// Convert a MIDI note ID to a frequency at A4 = 440 Hz equal temperament and middle C = note 60 =
/// C4.
#[inline]
pub fn midi_note_to_freq(note: u8) -> f32 {
    f32_midi_note_to_freq(note as f32)
}

/// The same as [`midi_note_to_freq()`], but for arbitrary note numbers including those outside of
/// the MIDI range. This also supports fractional note numbers, which is useful when working with
/// cents.
#[inline]
pub fn f32_midi_note_to_freq(note: f32) -> f32 {
    2.0f32.powf((note - 69.0) / 12.0) * 440.0
}

/// The inverse of [`f32_midi_note_to_freq()`]. This returns a fractional note number. Round to a
/// whole number, subtract that from the result, and multiply the fractional part by 100 to get the
/// number of cents.
#[inline]
pub fn freq_to_midi_note(freq: f32) -> f32 {
    ((freq / 440.0).log2() * 12.0) + 69.0
}

#[cfg(test)]
mod tests {
    mod db_gain_conversion {
        use super::super::*;

        #[test]
        fn converts_positive_db_to_gain() {
            let result = db_to_gain(3.0);
            assert_eq!(result, 1.4125376, "3dB should convert to gain of ~1.41");
        }

        #[test]
        fn converts_negative_db_to_attenuation() {
            let result = db_to_gain(-3.0);
            let expected = 1.4125376f32.recip();
            assert_eq!(result, expected, "-3dB should convert to reciprocal of +3dB gain");
        }

        #[test]
        fn converts_very_negative_db_to_zero_gain() {
            let result = db_to_gain(-100.0);
            assert_eq!(result, 0.0, "Very negative dB should result in zero gain (silence)");
        }

        #[test]
        fn converts_gain_above_unity_to_positive_db() {
            let result = gain_to_db(4.0);
            assert_eq!(result, 12.041201, "Gain of 4.0 should be approximately +12dB");
        }

        #[test]
        fn converts_gain_below_unity_to_negative_db() {
            let result = gain_to_db(0.25);
            assert_eq!(result, -12.041201, "Gain of 0.25 should be approximately -12dB");
        }

        #[test]
        fn converts_zero_gain_to_minus_infinity() {
            let result = gain_to_db(0.0);
            assert_eq!(result, MINUS_INFINITY_DB, "Zero gain should map to minus infinity dB");
        }

        #[test]
        fn converts_negative_gain_to_minus_infinity() {
            let result = gain_to_db(-2.0);
            assert_eq!(result, MINUS_INFINITY_DB, "Negative gain (invalid) should map to minus infinity dB");
        }
    }

    mod fast_db_gain_conversion {
        use super::super::*;

        const EPSILON: f32 = 1e-7;

        #[test]
        fn fast_positive_db_matches_accurate_conversion() {
            let accurate = db_to_gain(3.0);
            let fast = db_to_gain_fast_branching(3.0);
            approx::assert_relative_eq!(accurate, fast, epsilon = EPSILON);
        }

        #[test]
        fn fast_negative_db_matches_accurate_conversion() {
            let accurate = db_to_gain(-3.0);
            let fast = db_to_gain_fast_branching(-3.0);
            approx::assert_relative_eq!(accurate, fast, epsilon = EPSILON);
        }

        #[test]
        fn fast_very_negative_db_matches_accurate_conversion() {
            let accurate = db_to_gain(-100.0);
            let fast = db_to_gain_fast_branching(-100.0);
            approx::assert_relative_eq!(accurate, fast, epsilon = EPSILON);
        }

        #[test]
        fn fast_gain_to_db_matches_for_high_gain() {
            let accurate = gain_to_db(4.0);
            let fast = gain_to_db_fast(4.0);
            approx::assert_relative_eq!(accurate, fast, epsilon = EPSILON);
        }

        #[test]
        fn fast_gain_to_db_matches_for_attenuation() {
            let accurate = gain_to_db(0.25);
            let fast = gain_to_db_fast(0.25);
            approx::assert_relative_eq!(accurate, fast, epsilon = EPSILON);
        }

        #[test]
        fn fast_gain_to_db_handles_zero_gain() {
            let accurate = gain_to_db(0.0);
            let fast = gain_to_db_fast(0.0);
            approx::assert_relative_eq!(accurate, fast, epsilon = EPSILON);
        }

        #[test]
        fn fast_gain_to_db_handles_negative_gain() {
            let accurate = gain_to_db(-2.0);
            let fast = gain_to_db_fast(-2.0);
            approx::assert_relative_eq!(accurate, fast, epsilon = EPSILON);
        }
    }
}
