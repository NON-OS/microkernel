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

//! The MTU exchange as the client (Vol 3, Part F, 3.4.2): asked once,
//! first, offering 247; a server's answer from 23 to the offer becomes
//! the bearer's ATT_MTU, and the Report Map is read in its larger pieces.

use crate::map::maps::KEYBOARD;
use crate::sim::report_devices::{ready, report_keyboard, MODE};
use crate::sim::{Peripheral, Run};

/// The report keyboard's map padded with items that change nothing, so
/// it is longer than one response at 247.
pub fn long_map() -> Vec<u8> {
    [&KEYBOARD[..], &[0x34; 210][..]].concat()
}

/// A run against a server whose Rx MTU is `rx`; the Read Blob offsets.
pub fn run_at(rx: u16) -> (Run, Vec<u16>) {
    let mut peer = Peripheral::new(report_keyboard(&long_map()));
    peer.rx_mtu = rx;
    let mut run = Run::with(peer);
    ready(&mut run);
    let blobs = run.sent.iter().filter(|p| p[0] == 0x0c);
    let offsets = blobs.map(|p| u16::from_le_bytes([p[3], p[4]])).collect();
    (run, offsets)
}

#[test]
fn the_exchange_is_asked_once_and_first() {
    let (run, _) = run_at(247);
    assert_eq!(run.sent[0], [0x02, 0xf7, 0x00]);
    assert_eq!(run.sent.iter().filter(|p| p[0] == 0x02).count(), 1);
}

#[test]
fn an_answer_equal_to_the_offer_is_taken() {
    let (run, offsets) = run_at(247);
    assert_eq!(run.host.mtu(), 247);
    assert_eq!(offsets, [246], "300 octets: 246 by Read, 54 by one Read Blob");
    assert_eq!(run.peer.writes[0], (MODE, vec![1], true), "and the map is used");
}

#[test]
fn a_smaller_answer_is_the_mtu() {
    let (run, offsets) = run_at(100);
    assert_eq!(run.host.mtu(), 100);
    assert_eq!(offsets, [99, 198, 297]);
}

#[test]
fn the_least_answer_keeps_the_default() {
    let (run, offsets) = run_at(23);
    assert_eq!(run.host.mtu(), 23);
    assert_eq!(offsets.len(), 300 / 22);
}
