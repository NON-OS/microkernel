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

//! What any bytes a guest hands TCSETS2 must leave (console/termios2.rs
//! from2): refused unless they are 44 bytes; else settings that are the
//! guest's own but for c_cflag's two line rates, each the code the guest
//! named, or for a rate by number (BOTHER) the named rate of that speed or
//! the one there was. Settings read back by TCGETS2 and set again come
//! back unchanged. The fuzz target and the sweep below hold every input to
//! this.

use super::termios::{Termios, SIZE};
use super::termios2::{from2, to2, SIZE2};

// Hand-synced with console/termios2.rs.
const CBAUD: u32 = 0o10017;
const BOTHER: u32 = 0o10000;
const CIBAUD: u32 = CBAUD << 16;

fn word(t: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([t[at], t[at + 1], t[at + 2], t[at + 3]])
}

/// The settings TCSETS2 takes from `raw` over `was`, held to the rules.
pub fn check_set(raw: &[u8], was: &Termios) -> Option<Termios> {
    let Some(t) = from2(raw, was) else {
        assert_ne!(raw.len(), SIZE2, "44 bytes refused");
        return None;
    };
    assert_eq!(raw.len(), SIZE2, "settings taken from {} bytes", raw.len());
    assert_eq!(t[..8], raw[..8], "a flag word other than c_cflag changed");
    assert_eq!(t[12..], raw[12..SIZE], "c_line or c_cc changed");
    let (asked, got, old) = (word(raw, 8), word(&t, 8), word(was, 8));
    let rates = CBAUD | CIBAUD;
    assert_eq!(got & !rates, asked & !rates, "a c_cflag bit past the rates changed");
    for (mask, shift) in [(CBAUD, 0), (CIBAUD, 16)] {
        let (a, g, o) = ((asked & mask) >> shift, (got & mask) >> shift, (old & mask) >> shift);
        if a != BOTHER {
            assert_eq!(g, a, "a named rate not kept");
        } else {
            assert!(g == o || rate_named(g), "a rate by number is neither named nor the old");
        }
    }
    if no_bother(&t) {
        assert_eq!(from2(&to2(&t), was), Some(t), "settings read back and set again moved");
    }
    Some(t)
}

/// Whether `code` is a named rate's, which to2 reads as a speed again.
fn rate_named(code: u32) -> bool {
    code != BOTHER && (code & !CBAUD) == 0 && !(0o20..=BOTHER).contains(&code)
}

fn no_bother(t: &Termios) -> bool {
    let cflag = word(t, 8);
    cflag & CBAUD != BOTHER && (cflag & CIBAUD) >> 16 != BOTHER
}

/// Any bytes as a fuzz input: the first SIZE the settings there were, the
/// rest what TCSETS2 is handed.
pub fn check_bytes(data: &[u8]) {
    let mut was = [0u8; SIZE];
    let n = data.len().min(SIZE);
    was[..n].copy_from_slice(&data[..n]);
    let _ = check_set(&data[n..], &was);
}
