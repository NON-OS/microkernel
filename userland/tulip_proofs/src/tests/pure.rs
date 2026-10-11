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

//! The layouts the part depends on, against the manuals.

use crate::init::reset::stopped;
use crate::init::running;
use crate::init::setup_frame::encode;
use crate::queue::rx_consume::frame_len;
use crate::queue::rx_prime::rx_control;
use crate::queue::tx_post::tx_control;

#[test]
fn a_frame_length_drops_the_fcs_and_refuses_the_rest() {
    let whole = (1 << 9) | (1 << 8);
    let fl = |n: u32| n << 16;
    assert_eq!(frame_len(whole | fl(64)), 60);
    assert_eq!(frame_len(whole | fl(1518)), 1514);
    assert_eq!(frame_len(whole | fl(1519)), 0, "longer than an untagged frame");
    assert_eq!(frame_len(whole | fl(17)), 0, "a runt");
    assert_eq!(frame_len(whole | fl(64) | 1 << 15), 0, "errored");
    assert_eq!(frame_len(1 << 9 | fl(64)), 0, "no last descriptor");
    assert_eq!(frame_len(1 << 8 | fl(64)), 0, "no first descriptor");
}

// 21140A manual, setup frame for perfect filtering: each address as three
// sixteen-bit halves, low half of each longword.
#[test]
fn the_setup_frame_is_laid_out_as_the_manual_says() {
    let mac = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
    let f = encode(&mac);
    assert_eq!(&f[0..12], &[0x02, 0x11, 0, 0, 0x22, 0x33, 0, 0, 0x44, 0x55, 0, 0]);
    assert_eq!(&f[12..24], &[0xFF, 0xFF, 0, 0, 0xFF, 0xFF, 0, 0, 0xFF, 0xFF, 0, 0]);
    assert_eq!(&f[180..192], &f[0..12]);
}

#[test]
fn only_the_last_descriptor_of_each_ring_ends_it() {
    assert_eq!((rx_control(0), rx_control(31)), (1536, 1536 | 1 << 25));
    assert_eq!(tx_control(30, 60, 3 << 29), 60 | 3 << 29);
    assert_eq!(tx_control(31, 60, 3 << 29), 60 | 3 << 29 | 1 << 25);
}

#[test]
fn process_states_read_out_of_csr5() {
    assert!(stopped(0xF000_0000) && !running(0xF000_0000));
    assert!(running(3 << 17 | 6 << 20));
    assert!(!running(3 << 17 | 6 << 20 | 1 << 13), "a fatal bus error");
    assert!(!running(3 << 17) && !stopped(3 << 17));
}
