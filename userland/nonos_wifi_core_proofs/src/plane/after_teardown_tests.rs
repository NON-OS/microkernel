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

//! Once teardown begins, nothing is delivered up and nothing more leaves. A
//! deauthentication and a loss of the AP each end the link; after each, the
//! AP's continued chatter reaches no one and every transmit is refused. The
//! chip's restart (E-53) is the third teardown, in `restart_tests`.

use super::drive::{rx, tx, up};
use super::rig::{data_frames, fresh};
use crate::soft::join::{deauth_from_ap, ticks, PASS, SSID};
use crate::soft::rig::Station;
use crate::soft::wire::{code, connect};

fn joined() -> Station {
    let mut st = fresh();
    assert_eq!(code(&connect(&mut st, SSID, PASS, 0)), 0);
    st
}

pub(super) fn torn_down_is_quiet(st: &mut Station) {
    assert!(!up(st), "the link is down");
    let before = data_frames(st);
    for tag in 0..8u8 {
        assert!(tx(st, tag) < 0, "transmit refused after teardown");
    }
    ticks(st, 60); // the AP keeps chattering
    assert!(!rx(st), "no frame delivered after teardown");
    assert_eq!(data_frames(st), before, "no data frame left after teardown");
}

#[test]
fn a_deauthentication_stops_the_plane() {
    let mut st = joined();
    deauth_from_ap(&mut st);
    ticks(&mut st, 4);
    torn_down_is_quiet(&mut st);
}

#[test]
fn a_loss_of_the_ap_stops_the_plane() {
    let mut st = joined();
    st.radio_mut().unwrap().ap.on = false; // the AP goes silent
    for _ in 0..11_000 {
        st.tick();
        if !up(&mut st) {
            break;
        }
    }
    torn_down_is_quiet(&mut st);
}
