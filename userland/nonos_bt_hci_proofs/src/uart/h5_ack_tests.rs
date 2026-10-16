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

//! Acknowledgements against a hostile peer (Vol 4, Part D, "Reliable
//! Packets"): one naming packets never sent covers nothing, and a peer
//! that never acknowledges ends the link after `LOST_MS`, not before.

use super::h5_link_tests::active_with_window_one;
use super::h5_pair::RETX_MS;
use nonos_bt_hci::uart::h5::LOST_MS;

#[test]
fn an_ack_for_packets_never_sent_is_passed_over() {
    let mut h5 = active_with_window_one();
    assert!(h5.send(1, &[0x03, 0x0c, 0x00]));
    h5.poll_out(3, 250);
    for ack in [0u8, 2, 3, 4, 5, 6, 7] {
        let b0 = ack << 3;
        h5.feed(&[0xc0, b0, 0x00, 0x00, !b0, 0xc0], 4);
        assert_eq!(h5.unacked(), 1, "ack {ack} covers nothing sent");
    }
    h5.feed(&[0xc0, 0x08, 0x00, 0x00, 0xf7, 0xc0], 5);
    assert_eq!(h5.unacked(), 0, "ack 1 covers seq 0");
}

#[test]
fn a_peer_that_never_acknowledges_ends_the_link_after_lost_ms() {
    let mut h5 = active_with_window_one();
    assert!(h5.send(1, &[0x03, 0x0c, 0x00]));
    let (mut resends, mut lost_at) = (0, None);
    for now in 3..10_000u64 {
        if h5.poll_out(now, RETX_MS).is_some() {
            resends += 1;
        }
        if h5.lost(now) {
            lost_at = Some(now);
            break;
        }
    }
    assert_eq!(lost_at, Some(3 + LOST_MS), "sent at 3, lost exactly LOST_MS later");
    assert_eq!(resends, 1 + LOST_MS / RETX_MS, "sent again every RETX_MS meanwhile");
}
