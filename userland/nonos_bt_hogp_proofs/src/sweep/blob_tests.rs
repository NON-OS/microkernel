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
//! A Report Map longer than one response (Vol 3, Part G, 4.8.3): read with
//! a Read, then Read Blobs at the length so far while each answer fills a
//! response at the default ATT_MTU of 23, and no further than HIDS 1.0's
//! 512 octets. The offsets asked for are the ones that rule gives.

use super::rng::each;
use crate::sim::report_devices::{report_keyboard_with, MAP};
use crate::sim::Run;

/// The Read Blob offsets the rule asks for, for a map of `len` octets.
fn offsets(len: usize) -> Vec<u16> {
    let (mut total, mut part, mut out) = (0, len.min(22), Vec::new());
    while total + part <= 512 {
        total += part;
        if part < 22 {
            break;
        }
        out.push(total as u16);
        part = (len - total).min(22);
    }
    out
}

#[test]
fn a_long_map_is_read_in_blobs_to_its_end_or_the_bound() {
    each(|rng| {
        let len = 1 + rng.below(600);
        let map = rng.bytes(len);
        let mut run = Run::new(report_keyboard_with(&map, [&[1, 1], &[2, 1], &[1, 2]]));
        run.go();
        let blobs: Vec<u16> = run
            .sent
            .iter()
            .filter(|p| p[0] == 0x0c && p[1..3] == MAP.to_le_bytes())
            .map(|p| u16::from_le_bytes([p[3], p[4]]))
            .collect();
        assert_eq!(blobs, offsets(len), "a map of {len} octets");
    });
}
