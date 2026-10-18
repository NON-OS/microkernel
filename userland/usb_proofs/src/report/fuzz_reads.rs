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

//! What the report descriptor fuzz checks of every layout it gets: where
//! its fields end, and what hostile reports read through it come to.

use crate::descriptors::report::{Layout, NONE};
use crate::hid::report::keys::{keys_report, media_report};
use crate::hid::report::pointer::{pointer_report, Motion};
use crate::rng::Rng;

/// Every field end of the layout's parts, in bits.
pub fn ends(l: &Layout) -> Vec<u32> {
    let bit = |at: u16, size: u32| if at == NONE { 0 } else { u32::from(at) + size };
    let mut out = Vec::new();
    if let Some(k) = l.keys {
        out.extend(k.mods.iter().map(|&a| bit(a, 1)));
        out.extend(k.slots.map(|s| bit(s.at, u32::from(s.size) * u32::from(s.count))));
    }
    for m in l.media.iter().flatten() {
        out.extend(m.bits.iter().map(|&a| bit(a, 1)));
        out.extend(m.slots.map(|s| bit(s.at, u32::from(s.size) * u32::from(s.count))));
    }
    if let Some(p) = l.pointer {
        out.extend(p.buttons.iter().chain([&p.tip, &p.range]).map(|&a| bit(a, 1)));
        out.extend([p.x, p.y, p.wheel].iter().flatten().map(|f| bit(f.at, u32::from(f.size))));
    }
    out
}

pub fn check_reads(l: &Layout, rng: &mut Rng, round: u32) {
    for _ in 0..4 {
        let len = rng.below(9);
        let body = rng.bytes(len);
        let _ = l.keys.and_then(|k| keys_report(&k, &body));
        l.media.iter().flatten().for_each(|m| _ = media_report(m, &body));
        match l.pointer.and_then(|p| pointer_report(&p, &body)) {
            Some(Motion::Absolute(t)) => {
                assert!(
                    (0..=0x7fff).contains(&t.x) && (0..=0x7fff).contains(&t.y),
                    "round {round}"
                );
                assert!(t.buttons <= 0x1f && (-128..=127).contains(&t.wheel), "round {round}");
            }
            Some(Motion::Relative(ev)) => assert!(ev.buttons <= 0x1f, "round {round}"),
            None => {}
        }
    }
}
