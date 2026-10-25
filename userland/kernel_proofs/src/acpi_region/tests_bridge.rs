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

//! MkAcpiPci's bridge narrowing: on a bridge every write touching 0x18 to
//! 0x2F is refused and a Command write may not change bus mastering; a
//! function that is no bridge is never narrowed.

use super::rule::{bridge_write, Refusal, ROUTING};

fn writes() -> impl Iterator<Item = (u16, u8)> {
    [1u8, 2, 4].into_iter().flat_map(|w| (0..0x100u16).step_by(w.into()).map(move |at| (at, w)))
}

#[test]
fn bridges_keep_their_routing() {
    for header in [0x01u8, 0x81, 0x02, 0x82] {
        for (at, w) in writes() {
            let touches = at < ROUTING.1 && ROUTING.0 < at + u16::from(w);
            let command = at <= 4 && 4 < at + u16::from(w);
            let bm = 4u32.checked_shl(u32::from(4u16.saturating_sub(at)) * 8).unwrap_or(0);
            for value in [0u32, 0xFFFF_FFFF, bm] {
                let got = bridge_write(header, at, w, value, 0);
                if touches {
                    assert_eq!(got, Err(Refusal::Bridge), "{header:#x} {at:#x}/{w}");
                } else if !command {
                    assert_eq!(got, Ok(()), "{header:#x} {at:#x}/{w}");
                }
            }
        }
    }
}

#[test]
fn bus_mastering_never_changes() {
    for header in [0x01u8, 0x02] {
        for (at, w) in writes().filter(|(at, w)| *at <= 4 && 4 < *at + u16::from(*w)) {
            let shift = u32::from(4 - at) * 8;
            for (now, asked) in [(0u16, 0u32), (4, 4), (0, 4), (4, 0), (0x0106, 0x02), (0x07, 0x07)]
            {
                let got = bridge_write(header, at, w, (asked | 0x3) << shift, now);
                let same = u32::from(now) & 4 == asked & 4;
                assert_eq!(got.is_ok(), same, "{at:#x}/{w} now {now:#x} asked {asked:#x}");
            }
        }
    }
}

#[test]
fn other_functions_are_not_narrowed() {
    for header in [0x00u8, 0x80, 0x7F, 0x03] {
        for (at, w) in writes() {
            assert_eq!(bridge_write(header, at, w, u32::MAX, 0), Ok(()), "{header:#x}");
        }
    }
}
