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

//! The endpoint number and packet size a binding takes, and the cap on how
//! many bindings one configuration makes.

use super::KBD;
use crate::config_blob::{config, summaries, HID, INTERRUPT};
use crate::descriptors::hid_bindings;
use crate::protocol::MAX_HID_BINDINGS;

#[test]
fn the_packet_size_is_read_without_the_high_bandwidth_bits() {
    let raw = config().iface(0, HID, 1, 1).ep(0x81, INTERRUPT, 0x1808, 10).build();
    assert_eq!(summaries(&hid_bindings(&raw).expect("ok")), vec![KBD]);
}

#[test]
fn endpoint_zero_is_never_bound() {
    // Endpoint 0 is the control pipe; a record naming it as an interrupt IN
    // endpoint would have the controller driver reconfigure that pipe.
    for address in (0x80u8..=0xff).filter(|a| a & 0x0f == 0) {
        let raw = config().iface(0, HID, 1, 1).ep(address, INTERRUPT, 8, 10).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0), "address {address:#x}");
    }
    for number in 1u8..=15 {
        let raw = config().iface(0, HID, 1, 1).ep(0x80 | number, INTERRUPT, 8, 10).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(1), "endpoint {number}");
    }
}

#[test]
fn a_packet_size_binds_only_when_it_holds_one_report_and_fits_an_interrupt_endpoint() {
    // Keyboard reports are 8 bytes and mouse reports at least 3; any other
    // interface's report descriptor says how long its reports are, and its
    // packet need only hold a byte until then. No interrupt endpoint moves
    // more than 1024.
    for (subclass, protocol, least) in [(1u8, 1u8, 8u16), (1, 2, 3), (0, 0, 1)] {
        for wmax in 0..=u16::MAX {
            let raw =
                config().iface(0, HID, subclass, protocol).ep(0x81, INTERRUPT, wmax, 10).build();
            // Bits 12:11 count transactions and are read past; 15:13 are
            // reserved and refuse the endpoint (the the ruling, B 75).
            let size = wmax & 0x07ff;
            let want = usize::from(wmax >> 13 == 0 && (least..=1024).contains(&size));
            let got = hid_bindings(&raw).expect("well formed").len();
            assert_eq!(got, want, "protocol {protocol} wMaxPacketSize {wmax:#x}");
        }
    }
}

#[test]
fn bindings_stop_at_the_cap_however_many_interfaces_follow() {
    let mut blob = config();
    for n in 0..40u8 {
        blob = blob.iface(n, HID, 1, 1).ep(0x81, INTERRUPT, 8, 10);
    }
    let got = hid_bindings(&blob.build()).expect("ok");
    assert_eq!(got.len(), MAX_HID_BINDINGS);
    let numbers: Vec<u8> = got.iter().map(|b| b.interface_number).collect();
    assert_eq!(numbers, (0..MAX_HID_BINDINGS as u8).collect::<Vec<_>>());
}
