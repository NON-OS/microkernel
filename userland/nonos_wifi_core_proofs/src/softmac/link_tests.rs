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

//! A live session ends when the AP deauthenticates the station, when the AP
//! falls silent, or on the person's disconnect, which tells the AP; each time
//! the link reads down and the background scan resumes.

use crate::ap_sim::AA;
use crate::soft::join::{joined, ticks};
use crate::soft::rig::Station;
use crate::soft::wire::{code, control, netif};
use nonos_wifi_core::softmac::session::LOSS_MS;

fn link_up(st: &mut Station) -> bool {
    netif(st, 2, &[]).1 == [1]
}

#[test]
fn the_aps_deauthentication_ends_the_session() {
    let mut st = joined();
    assert!(link_up(&mut st));
    crate::soft::join::deauth_from_ap(&mut st);
    ticks(&mut st, 1);
    assert!(!link_up(&mut st));
    assert_eq!(control(&mut st, 5, &[])[0], 0);
    assert_eq!(st.radio().unwrap().bss, None);
    let steps = st.scanner().steps;
    ticks(&mut st, 3);
    assert_eq!(st.scanner().steps, steps + 3, "the scan resumed");
}

#[test]
fn a_silent_ap_ends_the_session_after_the_loss_interval() {
    let mut st = joined();
    st.radio_mut().unwrap().ap.on = false;
    let start = st.radio().unwrap().clock.get();
    while link_up(&mut st) {
        ticks(&mut st, 1);
        assert!(st.radio().unwrap().clock.get() < start + 2 * LOSS_MS, "never declared lost");
    }
    assert!(st.radio().unwrap().clock.get() >= start + LOSS_MS, "not before the interval");
}

#[test]
fn a_disconnect_tells_the_ap_and_ends_the_session() {
    let mut st = joined();
    assert_eq!(code(&control(&mut st, 2, &[])), 0);
    let last = st.radio().unwrap().sent.last().unwrap().clone();
    assert_eq!((last[0], &last[4..10], &last[24..26]), (0xc0, &AA[..], &[3u8, 0][..]));
    assert!(!link_up(&mut st));
    assert_eq!(netif(&mut st, 4, &[0u8; 20]).0, -5, "nothing goes out without a link");
}
