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

//! CAPS and MODES round trips and refusals.

use crate::body::caps::{Caps, HW_CURSOR, MODE_RANGE, MODE_SET};
use crate::body::mode::{Mode, FROM_EDID, PREFERRED};
use crate::body::modes::Modes;
use crate::errno::EINVAL;
use crate::kind;

#[test]
fn caps_round_trips_and_refuses_an_unusable_adapter() {
    let c = Caps {
        kind: kind::INTEL,
        outputs: 3,
        flags: MODE_SET | MODE_RANGE | HW_CURSOR,
        max_width: 5120,
        max_height: 3200,
        width_align: 64,
        surface_budget: 8192 * 4096,
        physical_mm: 344 | 194 << 16,
    };
    let b = c.encode();
    assert_eq!(&b[..2], &7u16.to_le_bytes());
    assert_eq!(Caps::decode(&b), Ok(c));
    assert!(c.has(HW_CURSOR) && !c.has(1 << 4));
    assert_eq!(Caps::decode(&Caps { outputs: 0, ..c }.encode()), Err(EINVAL));
    assert_eq!(Caps::decode(&Caps { width_align: 0, ..c }.encode()), Err(EINVAL));
    assert_eq!(Caps::decode(&b[..31]), Err(EINVAL));
}

#[test]
fn modes_carry_up_to_twenty_and_no_more() {
    let mut m = Modes { len: 20, ..Modes::default() };
    for (i, s) in m.list.iter_mut().enumerate() {
        *s = Mode { width: 640 + i as u32, height: 480, flags: PREFERRED | FROM_EDID };
    }
    let mut out = [0u8; 252];
    assert_eq!(m.encode(&mut out), Ok(244));
    assert_eq!(Modes::decode(&out[..244]), Ok(m));
    assert_eq!(Modes::decode(&out[..243]), Err(EINVAL), "count past the body");
    out[0] = 21;
    assert_eq!(Modes::decode(&out), Err(EINVAL));
    assert_eq!(Modes::decode(&[0, 0, 0, 0]).unwrap().len, 0);
    assert_eq!(m.encode(&mut [0u8; 243]), Err(EINVAL), "no room");
}
