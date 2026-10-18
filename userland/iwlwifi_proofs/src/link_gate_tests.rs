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

//! Which requests the driver hands to the link gate (server/link_serve.rs,
//! nonos_wifi_core::netif::gate): every link frame, whatever its op, and no
//! request of the Wi-Fi control family, so settings and the setup wizard keep
//! scan, join and status while the link is net.core's alone.

use crate::control::{route, Route};
use nonos_wifi_core::netif::gate::is_link;
use nonos_wifi_core::netif::wire::{
    HDR_LEN, MAGIC_NNET, OP_LINK_STATUS, OP_RX_PACKET, OP_TX_PACKET,
};

fn control(op: u16) -> Vec<u8> {
    let mut r = 0x5749_4649u32.to_le_bytes().to_vec();
    r.extend_from_slice(&op.to_le_bytes());
    r.extend_from_slice(&7u32.to_le_bytes());
    r
}

fn link(op: u16) -> Vec<u8> {
    let mut r = vec![0u8; HDR_LEN];
    r[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    r[6..8].copy_from_slice(&op.to_le_bytes());
    r
}

#[test]
fn every_link_frame_goes_to_the_gate() {
    for op in [OP_LINK_STATUS, OP_TX_PACKET, OP_RX_PACKET, 0, 0xFFFF] {
        for can_join in [true, false] {
            assert_eq!(route(&link(op), can_join), Route::Link, "op {op}");
        }
        assert!(is_link(&link(op)));
    }
}

#[test]
fn the_control_family_never_does() {
    // scan, status and every other control op; connect, disconnect and link.
    for op in 0..=8u16 {
        let req = control(op);
        assert!(!is_link(&req), "op {op}");
        for can_join in [true, false] {
            let r = route(&req, can_join);
            assert!(matches!(r, Route::Control | Route::Join(_)), "op {op}: {r:?}");
        }
    }
}
