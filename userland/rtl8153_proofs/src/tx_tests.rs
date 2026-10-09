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

//! TX transfers against r8152's struct tx_desc: opts1 the length with
//! TX_FS (bit 31) and TX_LS (bit 30), opts2 zero, the frame after it.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Nic;

use crate::chip::bound;
use crate::r8153::tx::{put_frame, TX_DESC};

#[test]
fn a_frame_goes_behind_its_descriptor() {
    let mut out = [0u8; 2048];
    let frame = [0xab; 60];
    assert_eq!(put_frame(&frame, &mut out), Some(68));
    assert_eq!(out[..TX_DESC], [60, 0, 0, 0xc0, 0, 0, 0, 0]);
    assert_eq!(out[TX_DESC..68], frame);
    assert_eq!(put_frame(&[0; 1514], &mut out), Some(1522));
    assert_eq!(out[..4], [0xea, 0x05, 0, 0xc0]);
}

#[test]
fn a_transfer_filling_its_last_packet_is_not_padded() {
    let mut out = [0u8; 2048];
    for (frame, n) in [(56, 64), (504, 512), (1016, 1024)] {
        assert_eq!(put_frame(&vec![1; frame], &mut out), Some(n));
    }
}

#[test]
fn frames_outside_ethernet_sizes_are_refused() {
    let mut out = [0u8; 2048];
    assert_eq!(put_frame(&[0; 13], &mut out), None);
    assert_eq!(put_frame(&[0; 1515], &mut out), None);
    assert_eq!(put_frame(&[0; 100], &mut out[..107]), None);
}

#[test]
fn send_is_one_bulk_out_and_a_stall_is_cleared() {
    let (mut nic, bus, _) = bound(0x5c10);
    nic.send(&[7u8; 504]).unwrap();
    let last = bus.0.borrow().calls.last().cloned();
    let mut want = vec![0xf8, 0x01, 0, 0xc0, 0, 0, 0, 0];
    want.extend([7u8; 504]);
    assert_eq!(last, Some(Call::BulkOut(want)));
    bus.0.borrow_mut().bulk_out_fails = Some(-32);
    assert_eq!(nic.send(&[7u8; 60]), Err(-32));
    let calls = bus.0.borrow().calls.clone();
    assert_eq!(calls[calls.len() - 2], Call::ResetBulk(false));
    assert!(
        matches!(&calls[calls.len() - 1], Call::Out(s, _) if s.request == 0x01 && s.index == 0x02)
    );
    assert_eq!(nic.send(&[0; 1515]), Err(-22));
}
