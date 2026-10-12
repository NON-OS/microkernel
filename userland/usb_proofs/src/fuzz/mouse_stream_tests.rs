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

//! A hostile mouse stream through the boot decode: motion and buttons stay
//! inside their defined ranges, and every button event is one that changed.

use super::ROUNDS;
use crate::hid::button_changes::button_changes;
use crate::hid::mouse_report::{mouse_event, MOUSE_BUTTONS};
use crate::rng::Rng;

#[test]
fn a_hostile_mouse_stream_stays_inside_the_defined_motion_and_buttons() {
    let mut rng = Rng::seeded(0x5eed_0004);
    let mut buttons = 0u8;
    for round in 0..ROUNDS {
        let len = rng.below(9);
        let mut raw = rng.bytes(len);
        if len >= 3 && rng.one_in(3) {
            raw[1..].iter_mut().for_each(|b| *b = 0);
        }
        if len >= 1 && rng.one_in(3) {
            raw[0] = buttons | (rng.byte() & 0xe0);
        }
        let Some(ev) = mouse_event(&raw, buttons) else {
            let quiet =
                len >= 3 && raw[0] & 0x1f == buttons && raw[1..len.min(4)].iter().all(|&b| b == 0);
            assert!(len < 3 || quiet, "round {round}: {raw:02x?} made nothing");
            continue;
        };
        assert!(len >= 3, "round {round}");
        assert_eq!(ev.buttons & !MOUSE_BUTTONS, 0, "round {round}");
        assert_eq!((ev.dx, ev.dy), (i16::from(raw[1] as i8), i16::from(raw[2] as i8)));
        assert!((-128..=127).contains(&ev.dx) && (-128..=127).contains(&ev.dy));
        assert_eq!(ev.dz, if len > 3 { raw[3] as i8 } else { 0 }, "round {round}");
        check_buttons(buttons, ev.buttons, MOUSE_BUTTONS, 5, round);
        buttons = ev.buttons;
    }
}

/// Every button event between two button bytes names a button from 1 to
/// `top`, at most once, in the direction it went.
pub fn check_buttons(previous: u8, current: u8, mask: u8, top: u32, round: u32) {
    let mut seen = 0u8;
    button_changes(previous, current, mask, |button, down| {
        assert!((1..=top).contains(&button), "round {round}: button {button}");
        let bit = 1u8 << (button - 1);
        assert_eq!(seen & bit, 0, "round {round}: button {button} twice");
        seen |= bit;
        assert_eq!(down, current & bit != 0, "round {round}: button {button}");
    });
    assert_eq!(seen, (previous ^ current) & mask, "round {round}");
}
