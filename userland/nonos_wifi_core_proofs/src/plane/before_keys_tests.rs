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

//! No data frame leaves before the keys are in. A station that never joined
//! transmits nothing and receives nothing; and when a join succeeds, the AP
//! is keyed before the first accepted transmit, so the first data the stack
//! sends is already protected.

use super::drive::{rx, tx, up};
use super::rig::{data_frames, fresh};
use crate::soft::join::{PASS, SSID};
use crate::soft::wire::{code, connect};

#[test]
fn a_station_that_never_joined_moves_no_data() {
    let mut st = fresh();
    assert!(!up(&mut st), "no link before a join");
    for tag in 0..8u8 {
        assert!(tx(&mut st, tag) < 0, "transmit refused with no link");
    }
    assert!(!rx(&mut st), "nothing to receive with no link");
    assert_eq!(data_frames(&st), 0, "no data frame left the station");
}

#[test]
fn the_keys_are_in_before_the_first_data_leaves() {
    let mut st = fresh();
    assert_eq!(data_frames(&st), 0);
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0, "joined WPA2-PSK");
    // The handshake put the pairwise key in at the AP; only now can data go.
    assert!(st.radio().unwrap().ap.tk.is_some(), "the AP is keyed");
    assert!(up(&mut st), "the link is up once keyed");
    assert!(tx(&mut st, 0xAB) >= 0, "transmit accepted once keyed");
    let got = &st.radio().unwrap().ap.got;
    assert!(got.iter().any(|b| b.contains(&0xAB)), "the first data reached the AP sealed");
}
