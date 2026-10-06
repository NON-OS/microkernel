// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The master volume as the mixer applies it. Full plays every sample as it
//! was mixed, so nothing changes for anyone who never sets it; the curve
//! never gets louder as the level falls; 0 and mute are silence; and no
//! sample at any level leaves the i16 range or comes out louder than it went
//! in. The last is checked for every sample at every level, not sampled.

use nonos_audio_proto::{MasterVolume, VOLUME_MAX};

use crate::mixer::{Mixer, BYTES, SAMPLES};
use crate::volume::{gain, scale, Volume, UNITY};

fn at(level: u8) -> i32 {
    gain(MasterVolume { level, muted: false })
}

#[test]
fn full_is_unity_and_plays_every_sample_unchanged() {
    assert_eq!(at(VOLUME_MAX), UNITY);
    for s in i16::MIN..=i16::MAX {
        assert_eq!(scale(s, UNITY), s);
    }
    let v = Volume::new();
    assert_eq!((v.setting(), v.gain()), (MasterVolume::FULL, UNITY), "a service starts at full");
}

#[test]
fn zero_and_mute_are_silence() {
    assert_eq!(at(0), 0);
    for level in 0..=VOLUME_MAX {
        assert_eq!(gain(MasterVolume { level, muted: true }), 0, "muted at {level}");
    }
    for s in i16::MIN..=i16::MAX {
        assert_eq!(scale(s, 0), 0);
    }
}

#[test]
fn the_curve_rises_with_every_step_and_is_the_square_of_the_level() {
    for level in 0..VOLUME_MAX {
        assert!(at(level) < at(level + 1), "{level} to {}", level + 1);
    }
    for level in 0..=VOLUME_MAX {
        let exact = f64::from(level).powi(2) / 10_000.0 * f64::from(UNITY);
        assert!((f64::from(at(level)) - exact).abs() <= 0.5, "level {level}");
    }
    // Perceptual, not linear: half the level is a quarter of the amplitude
    // (-12 dB), and the lowest step still plays.
    assert_eq!(at(50), UNITY / 4);
    assert!(at(1) > 0);
}

#[test]
fn a_level_over_full_plays_at_full_never_louder() {
    for level in VOLUME_MAX..=u8::MAX {
        assert_eq!(at(level), UNITY);
    }
}

#[test]
fn no_sample_at_any_level_overflows_or_comes_out_louder() {
    for level in 0..=VOLUME_MAX {
        let g = at(level);
        for s in i16::MIN..=i16::MAX {
            let out = scale(s, g);
            let exact = (i64::from(s) * i64::from(g)) as f64 / f64::from(UNITY);
            assert!((f64::from(out) - exact).abs() <= 0.5, "{s} at {level}: {out}");
            assert!(i32::from(out).abs() <= i32::from(s).abs(), "{s} at {level} grew to {out}");
            assert!(out == 0 || (out < 0) == (s < 0), "{s} at {level} flipped to {out}");
        }
    }
}

#[test]
fn the_mixer_writes_the_mix_at_the_gain() {
    let mut m = Mixer::new();
    let mut a = [0i16; SAMPLES];
    let mut b = [0i16; SAMPLES];
    for i in 0..SAMPLES {
        a[i] = (i as i16).wrapping_mul(97);
        b[i] = if i % 2 == 0 { i16::MAX } else { i16::MIN };
    }
    m.add(&a);
    m.add(&b);
    for level in [0, 5, 50, 99, 100] {
        let g = at(level);
        let mut out = [0u8; BYTES];
        m.write_bytes(&mut out, g);
        for i in 0..SAMPLES {
            let mixed = a[i].saturating_add(b[i]);
            let got = i16::from_le_bytes([out[i * 2], out[i * 2 + 1]]);
            assert_eq!(got, scale(mixed, g), "sample {i} at {level}");
        }
    }
}

#[test]
fn setting_the_volume_moves_the_gain_with_it() {
    let mut v = Volume::new();
    v.set(MasterVolume { level: 40, muted: false });
    assert_eq!(v.gain(), at(40));
    v.set(MasterVolume { level: 40, muted: true });
    assert_eq!((v.setting().level, v.gain()), (40, 0), "mute keeps the level to come back to");
    v.set(MasterVolume { level: 40, muted: false });
    assert_eq!(v.gain(), at(40));
}
