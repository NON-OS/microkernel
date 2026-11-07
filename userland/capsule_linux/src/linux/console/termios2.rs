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

//! The settings as termios2 holds them, for TCGETS2 and TCSETS2: the
//! kernel's `struct termios` followed by the input and output speeds, 44
//! bytes. glibc 2.42 and later ask a terminal this way alone (tcgetattr,
//! tcsetattr), so every Debian-family program did, and was told ENOTTY:
//! SimpleX's terminal library took that for a broken terminal and stopped.
//! The speeds are what the c_cflag line rate says; a terminal here has no
//! line, so a rate set by number (BOTHER) is kept only when it is one of
//! the named rates, and otherwise the rate is left as it was, as a
//! pseudo-terminal's has no effect. Pure, so the host proofs hold it.

use super::termios::{Termios, SIZE};

/// `struct termios2`: `struct termios`, c_ispeed, c_ospeed.
pub const SIZE2: usize = SIZE + 8;

/// c_cflag's line rate, CBAUD, and BOTHER, the rate given by number.
const CBAUD: u32 = 0o10017;
const BOTHER: u32 = 0o10000;
/// The input rate, CIBAUD, which 0 says is the output rate.
const CIBAUD: u32 = CBAUD << 16;

/// Each named rate's code, B0 to B4000000, and its bits a second.
const RATES: [(u32, u32); 31] = [
    (0o0, 0),
    (0o1, 50),
    (0o2, 75),
    (0o3, 110),
    (0o4, 134),
    (0o5, 150),
    (0o6, 200),
    (0o7, 300),
    (0o10, 600),
    (0o11, 1200),
    (0o12, 1800),
    (0o13, 2400),
    (0o14, 4800),
    (0o15, 9600),
    (0o16, 19200),
    (0o17, 38400),
    (0o10001, 57600),
    (0o10002, 115200),
    (0o10003, 230400),
    (0o10004, 460800),
    (0o10005, 500000),
    (0o10006, 576000),
    (0o10007, 921600),
    (0o10010, 1000000),
    (0o10011, 1152000),
    (0o10012, 1500000),
    (0o10013, 2000000),
    (0o10014, 2500000),
    (0o10015, 3000000),
    (0o10016, 3500000),
    (0o10017, 4000000),
];

fn word(t: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([t[at], t[at + 1], t[at + 2], t[at + 3]])
}

fn rate(code: u32) -> u32 {
    RATES.iter().find(|(c, _)| *c == code).map_or(0, |(_, r)| *r)
}

fn code(rate: u32) -> Option<u32> {
    RATES.iter().find(|(_, r)| *r == rate).map(|(c, _)| *c)
}

/// `t` as termios2: its speeds the rates its c_cflag names.
pub fn to2(t: &Termios) -> [u8; SIZE2] {
    let cflag = word(t, 8);
    let out = rate(cflag & CBAUD);
    let input = match (cflag & CIBAUD) >> 16 {
        0 => out,
        c => rate(c),
    };
    let mut t2 = [0u8; SIZE2];
    t2[..SIZE].copy_from_slice(t);
    t2[SIZE..SIZE + 4].copy_from_slice(&input.to_le_bytes());
    t2[SIZE + 4..].copy_from_slice(&out.to_le_bytes());
    t2
}

/// The settings termios2 `raw` asks for, with the line rate `was` had where
/// it asks for one by a number no named rate has. None when `raw` is not
/// 44 bytes.
pub fn from2(raw: &[u8], was: &Termios) -> Option<Termios> {
    let raw: &[u8; SIZE2] = raw.try_into().ok()?;
    let mut t: Termios = raw[..SIZE].try_into().ok()?;
    let mut cflag = word(&t, 8);
    let kept = word(was, 8);
    let by_number = |bits: u32, speed: u32, old: u32| match bits {
        BOTHER => code(speed).unwrap_or(old),
        other => other,
    };
    let out = by_number(cflag & CBAUD, word(raw, SIZE + 4), kept & CBAUD);
    let input = by_number((cflag & CIBAUD) >> 16, word(raw, SIZE), (kept & CIBAUD) >> 16);
    cflag = (cflag & !(CBAUD | CIBAUD)) | out | (input << 16);
    t[8..12].copy_from_slice(&cflag.to_le_bytes());
    Some(t)
}
