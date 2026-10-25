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

//! Link establishment and the first reliable exchange, against a peer
//! whose every octet is laid out by hand (Vol 4, Part D). A CONFIG
//! RESPONSE offering a window of one and no CRC is what the link uses.

use nonos_bt_hci::uart::h5::H5;

pub const SYNC_RESPONSE: [u8; 8] = [0xc0, 0x00, 0x2f, 0x00, 0xd0, 0x02, 0x7d, 0xc0];
// Length 3 makes the checksum 0xc0, sent escaped.
const CONFIG: [u8; 10] = [0xc0, 0x00, 0x3f, 0x00, 0xdb, 0xdc, 0x03, 0xfc, 0x14, 0xc0];
const CONFIG_RESPONSE_W1: [u8; 10] = [0xc0, 0x00, 0x3f, 0x00, 0xdb, 0xdc, 0x04, 0x7b, 0x01, 0xc0];
// Reliable, seq 0, ack 1, type 4: Command Complete for Reset.
const EVENT: [u8; 12] = [0xc0, 0x88, 0x64, 0x00, 0x13, 0x0e, 0x04, 0x01, 0x03, 0x0c, 0x00, 0xc0];

pub fn active_with_window_one() -> H5 {
    let mut h5 = H5::default();
    h5.poll_out(0, 250);
    h5.feed(&SYNC_RESPONSE, 1);
    assert_eq!(h5.poll_out(1, 250), Some(CONFIG.to_vec()), "CONFIG: window 4, CRC, version 0");
    h5.feed(&CONFIG_RESPONSE_W1, 2);
    assert!(h5.active());
    h5
}

#[test]
fn nothing_but_link_control_is_read_before_the_link_is_active() {
    let mut h5 = H5::default();
    h5.feed(&EVENT, 0);
    assert!(h5.inbox.is_empty());
    h5.feed(&SYNC_RESPONSE, 0);
    h5.feed(&EVENT, 0);
    assert!(h5.inbox.is_empty() && !h5.active(), "Initialized is not Active");
}

#[test]
fn a_command_goes_out_reliable_and_the_event_back_is_acknowledged() {
    let mut h5 = active_with_window_one();
    assert!(h5.send(1, &[0x03, 0x0c, 0x00]));
    assert!(!h5.send(1, &[0x03, 0x0c, 0x00]), "a window of one");
    // Reliable, seq 0, ack 0, no CRC (the peer offered none), type 1.
    assert_eq!(
        h5.poll_out(3, 250),
        Some(vec![0xc0, 0x80, 0x31, 0x00, 0x4e, 0x03, 0x0c, 0x00, 0xc0])
    );
    h5.feed(&EVENT, 4);
    assert_eq!(h5.unacked(), 0, "its ack of 1 covers the command");
    assert_eq!(h5.inbox.pop_front(), Some((4, vec![0x0e, 0x04, 0x01, 0x03, 0x0c, 0x00])));
    // A bare acknowledgement: unreliable, ack 1, type 0, length 0.
    assert_eq!(h5.poll_out(5, 250), Some(vec![0xc0, 0x08, 0x00, 0x00, 0xf7, 0xc0]));
    assert_eq!(h5.poll_out(6, 250), None, "owed once");
    h5.feed(&EVENT, 7);
    assert!(h5.inbox.is_empty(), "the same number again is a resend, not a new event");
    assert!(h5.poll_out(8, 250).is_some(), "but it is acknowledged again");
}
