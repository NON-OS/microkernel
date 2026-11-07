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

//! What any bytes typed and any settings must leave (console/tty_rules.rs
//! map_input): each byte typed is one byte read, or none for a carriage
//! return under IGNCR, in the order typed; under ISTRIP each has its eighth
//! bit cleared first; a carriage return reads as a newline under ICRNL and
//! a newline as a carriage return under INLCR; every other byte reads as
//! typed. The fuzz target holds every input to this.

use super::tty_rules::map_input;

// Hand-synced with console/tty_rules.rs.
const ISTRIP: u32 = 0o40;
const INLCR: u32 = 0o100;
const IGNCR: u32 = 0o200;
const ICRNL: u32 = 0o400;
/// The settings' bytes map_input is handed: struct termios, 36 bytes.
const SETTINGS: usize = 36;

/// What one byte typed reads as under c_iflag `i`, or None when dropped.
fn one(i: u32, typed: u8) -> Option<u8> {
    let c = if i & ISTRIP != 0 { typed & 0x7f } else { typed };
    if c == b'\r' && i & IGNCR != 0 {
        return None;
    }
    Some(match c {
        b'\r' if i & ICRNL != 0 => b'\n',
        b'\n' if i & INLCR != 0 => b'\r',
        c => c,
    })
}

/// `typed` mapped under the settings `t`, held to the rules.
pub fn check_map(t: &[u8], typed: &[u8]) {
    let i = t.get(..4).map_or(0, |w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]));
    let want: Vec<u8> = typed.iter().filter_map(|&c| one(i, c)).collect();
    let mut got = typed.to_vec();
    let kept = map_input(t, &mut got);
    assert!(kept <= typed.len(), "{kept} bytes kept of {}", typed.len());
    assert_eq!(got[..kept], want[..], "bytes read are not those typed, mapped");
}

/// Any bytes as a fuzz input: the first 36 the settings, the rest typed.
pub fn check_bytes(data: &[u8]) {
    let n = data.len().min(SETTINGS);
    check_map(&data[..n], &data[n..]);
}
