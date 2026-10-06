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

//! The frames of each bulk IN queued and handed up one per recv, and a
//! stalled pipe brought back.

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Nic, Setup};

use crate::chip::bound;

/// One bulk IN with frames of the given sizes, each behind its rx_desc.
fn transfer(sizes: &[usize]) -> Vec<u8> {
    let mut t = Vec::new();
    for (i, &n) in sizes.iter().enumerate() {
        t.extend(((n + 4) as u32).to_le_bytes());
        t.resize(t.len() + 20, 0);
        t.extend(vec![i as u8 + 1; n]);
        t.extend([0; 4]);
        t.resize(t.len().next_multiple_of(8), 0);
    }
    t
}

#[test]
fn frames_of_one_transfer_are_handed_up_one_per_recv() {
    let (mut nic, bus, _) = bound(0x5c10);
    let t = transfer(&[60, 1514, 1600, 100]);
    bus.0.borrow_mut().bulk_in.extend([Ok(Some(t)), Ok(Some(transfer(&[70])))]);
    let mut out = [0u8; 1514];
    assert_eq!(nic.recv(&mut out), Ok(Some(60)));
    assert_eq!(out[0], 1);
    assert_eq!(nic.recv(&mut out), Ok(Some(1514)));
    // The 1600-byte frame is over VLAN_ETH_FRAME_LEN and was dropped.
    assert_eq!(nic.recv(&mut out), Ok(Some(100)));
    assert_eq!(out[0], 4);
    assert_eq!(bus.0.borrow().bulk_in.len(), 1, "no poll while frames wait");
    assert_eq!(nic.recv(&mut out[..60]), Ok(None), "too big for the caller: dropped");
    assert_eq!(nic.recv(&mut out), Ok(None));
}

#[test]
fn a_stalled_bulk_in_is_cleared_on_both_sides() {
    let (mut nic, bus, _) = bound(0x5c10);
    bus.0.borrow_mut().bulk_in.extend([Err(-32), Ok(Some(vec![0; 10]))]);
    let mut out = [0u8; 1514];
    assert_eq!(nic.recv(&mut out), Err(-32));
    let calls = bus.0.borrow().calls.clone();
    assert_eq!(calls[calls.len() - 2], Call::ResetBulk(true));
    assert_eq!(calls[calls.len() - 1], Call::Out(Setup::new(0x02, 0x01, 0, 0x81), vec![]));
    assert_eq!(nic.recv(&mut out), Ok(None), "a malformed transfer is dropped");
}
