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

//! A hostile stream of QEMU tablet reports: positions stay inside the
//! logical range, buttons inside the three declared, and every button
//! event is one that changed.

use super::qemu::tablet;
use crate::fuzz::mouse_stream_tests::check_buttons;
use crate::fuzz::ROUNDS;
use crate::rng::Rng;

#[test]
fn a_hostile_tablet_stream_stays_inside_the_logical_range_and_buttons() {
    let mut rng = Rng::seeded(0x5eed_0005);
    let mut buttons = 0u8;
    for round in 0..ROUNDS {
        let len = rng.below(9);
        let raw = rng.bytes(len);
        let Some(t) = tablet(&raw) else {
            assert!(len < 6, "round {round}: {raw:02x?} made nothing");
            continue;
        };
        assert!((0..=0x7fff).contains(&t.x) && (0..=0x7fff).contains(&t.y), "round {round}");
        assert_eq!(t.buttons & !0x07, 0, "round {round}");
        assert!((-128..=127).contains(&t.wheel), "round {round}");
        check_buttons(buttons, t.buttons, 0x07, 3, round);
        buttons = t.buttons;
    }
}
