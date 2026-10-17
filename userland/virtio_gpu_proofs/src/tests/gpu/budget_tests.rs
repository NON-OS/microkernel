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
//! What one display grant can hold bounds what is offered (G-78): CAPS'
//! budget is the broker's 8192 pages, MODES leaves out a size whose
//! surface would not fit (a host asking for one included), and the wire
//! server refuses a MODE_SET over it before the driver is asked.

use nonos_gfx_wire::body::mode::PREFERRED;
use nonos_gfx_wire::errno::EINVAL;
use nonos_gfx_wire::{ops, Caps, Modes};

use super::ask::{ask, words};
use super::kit::{raise, state};
use super::rig::rig;

const GRANT: u64 = 8192 * 4096;

fn modes(r: &mut super::rig::Rig) -> Vec<(u32, u32, u32)> {
    let m = Modes::decode(&ask(r, ops::MODES, 0, &[]).1).expect("modes");
    m.as_slice().iter().map(|m| (m.width, m.height, m.flags)).collect()
}

#[test]
fn no_mode_over_one_grant_is_offered_or_set() {
    let mut r = rig();
    let caps = Caps::decode(&ask(&mut r, ops::CAPS, 0, &[]).1).unwrap();
    assert_eq!(caps.surface_budget, GRANT);
    raise(&mut r, 0, (4096, 4096));
    state(&mut r, 0);
    let m = modes(&mut r);
    assert!(m.iter().all(|&(w, h, _)| w as u64 * h as u64 * 4 <= GRANT), "{m:?}");
    assert!(!m.iter().any(|&(w, h, _)| (w, h) == (4096, 4096)), "the host's ask left out");
    assert_eq!(ask(&mut r, ops::MODE_SET, 0, &words(&[4096, 4096])).0, EINVAL);
    assert_eq!(ask(&mut r, ops::MODE_SET, 0, &words(&[4096, 2048])).0, 0, "one grant whole");
    raise(&mut r, 0, (1920, 1080));
    state(&mut r, 0);
    let m = modes(&mut r);
    assert_eq!((m[0].0, m[0].1), (1920, 1080), "the host's ask that fits leads");
    assert_ne!(m[0].2 & PREFERRED, 0);
}

#[test]
fn edid_modes_are_held_to_the_grant_and_the_sides() {
    use crate::display::modes::{fits, within_grant};
    assert!(fits(4096, 2048) && !fits(4096, 2049), "one grant whole, not a row over");
    assert!(!fits(639, 480) && !fits(640, 479) && fits(640, 480));
    assert!(!fits(4097, 480), "a side over");
    assert!(within_grant(320, 200), "a host's small ask is offered");
    assert!(!within_grant(8192, 4096));
}
