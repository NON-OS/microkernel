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

//! What the input menu calls each input a display lists. The values are
//! MCCS's input source codes as recalled, not checked against the standard
//! here (unverified); a value not among them is named by its number, so a
//! monitor's own codes still read as something a person can match.

const NAMES: [(u8, &str); 18] = [
    (0x01, "VGA 1"),
    (0x02, "VGA 2"),
    (0x03, "DVI 1"),
    (0x04, "DVI 2"),
    (0x05, "Composite 1"),
    (0x06, "Composite 2"),
    (0x07, "S-Video 1"),
    (0x08, "S-Video 2"),
    (0x09, "Tuner 1"),
    (0x0A, "Tuner 2"),
    (0x0B, "Tuner 3"),
    (0x0C, "Component 1"),
    (0x0D, "Component 2"),
    (0x0E, "Component 3"),
    (0x0F, "DisplayPort 1"),
    (0x10, "DisplayPort 2"),
    (0x11, "HDMI 1"),
    (0x12, "HDMI 2"),
];

/// The input's name, written into `out`; its length.
pub fn name(input: u8, out: &mut [u8; 16]) -> usize {
    if let Some((_, n)) = NAMES.iter().find(|(v, _)| *v == input) {
        out[..n.len()].copy_from_slice(n.as_bytes());
        return n.len();
    }
    let hex = |d: u8| b"0123456789ABCDEF"[d as usize];
    let s = [b'I', b'n', b'p', b'u', b't', b' ', b'0', b'x', hex(input >> 4), hex(input & 15)];
    out[..s.len()].copy_from_slice(&s);
    s.len()
}
