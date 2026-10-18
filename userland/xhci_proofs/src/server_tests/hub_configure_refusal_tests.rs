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

//! OP_CONFIGURE_HUB values are the client's and are bounded before they
//! reach a context: 1 to 15 ports, a think time of 0 to 3, MTT 0 or 1, on
//! a slot addressed here, that is not low speed. Nothing refused reaches
//! the controller.

use super::bench::Bench;
use super::hub_bench::{HIGH, LOW};
use crate::protocol::{E_INVAL, OP_CONFIGURE_HUB};

#[test]
fn hub_values_out_of_range_or_a_slot_that_is_no_hub_are_refused() {
    let mut b = Bench::new(HIGH as u32);
    let hub = b.root_device();
    let enabled_only = b.enable_slot();
    let ran = b.ran().len();
    let hostile: [[u8; 4]; 11] = [
        [hub, 0, 0, 0],
        [hub, 16, 0, 0],
        [hub, 255, 0, 0],
        [hub, 4, 4, 0],
        [hub, 4, 255, 0],
        [hub, 4, 0, 2],
        [hub, 4, 0, 255],
        [enabled_only, 4, 0, 0],
        [0, 4, 0, 0],
        [9, 4, 0, 0],
        [255, 4, 0, 0],
    ];
    for h in hostile {
        assert_eq!(b.call(OP_CONFIGURE_HUB, &h).status, E_INVAL, "{h:?}");
    }
    for wrong in [&[hub, 4, 0][..], &[hub, 4, 0, 0, 0][..], &[][..]] {
        assert_eq!(b.call(OP_CONFIGURE_HUB, wrong).status, E_INVAL, "{} bytes", wrong.len());
    }
    assert_eq!(b.ran().len(), ran, "no command reached the controller");
    assert_eq!(b.configure_hub(hub, 15, 3, 1), 0, "the bounds themselves are fine");
}

#[test]
fn no_low_speed_device_is_a_hub() {
    let mut b = Bench::new(LOW as u32);
    let mouse = b.root_device();
    let ran = b.ran().len();
    assert_eq!(b.configure_hub(mouse, 4, 0, 0), E_INVAL);
    assert_eq!(b.ran().len(), ran);
}
