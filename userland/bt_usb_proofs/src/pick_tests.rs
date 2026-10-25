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

//! Which advertisers are HID devices to connect to: advertising data
//! walked as Vol 3, Part C, 11 lays it out, the HID service or a HID
//! appearance named, and only reports a central may answer.

use crate::link::pick::hid_advertiser;
use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_bt_wire::hci::reports::AdvReport;

fn report(event_type: u8, data: &[u8]) -> AdvReport<'_> {
    let addr = BdAddr([1, 2, 3, 4, 5, 0xc6]);
    AdvReport { event_type, addr_type: AddrType::Random, addr, data, rssi: -40 }
}

const FLAGS: [u8; 3] = [2, 0x01, 0x06];

#[test]
fn the_hid_service_in_either_uuid_list_is_a_hid_device() {
    let complete = [&FLAGS[..], &[5, 0x03, 0x0f, 0x18, 0x12, 0x18]].concat();
    let incomplete = [&FLAGS[..], &[3, 0x02, 0x12, 0x18]].concat();
    assert!(hid_advertiser(&report(0x00, &complete)));
    assert!(hid_advertiser(&report(0x01, &incomplete)));
    let battery_only = [&FLAGS[..], &[3, 0x03, 0x0f, 0x18]].concat();
    assert!(!hid_advertiser(&report(0x00, &battery_only)));
}

#[test]
fn a_hid_appearance_is_a_hid_device_and_others_are_not() {
    assert!(hid_advertiser(&report(0x00, &[3, 0x19, 0xc1, 0x03])), "keyboard");
    assert!(hid_advertiser(&report(0x00, &[3, 0x19, 0xc2, 0x03])), "mouse");
    assert!(!hid_advertiser(&report(0x00, &[3, 0x19, 0x40, 0x00])), "phone");
    // An appearance is two octets; a longer one is malformed, not a match.
    assert!(!hid_advertiser(&report(0x00, &[4, 0x19, 0xc1, 0x03, 0x00])));
}

#[test]
fn reports_a_central_may_not_answer_are_passed_over() {
    let hid = [&FLAGS[..], &[3, 0x03, 0x12, 0x18]].concat();
    for kind in [0x02, 0x03, 0x04] {
        assert!(!hid_advertiser(&report(kind, &hid)), "event type {kind}");
    }
}

#[test]
fn broken_data_ends_the_walk_without_a_match() {
    // The HID service sits after a structure that runs past the end, or
    // after a zero length that ends the data early.
    let past_end = [&[9, 0x09, b'k'][..], &[3, 0x03, 0x12, 0x18]].concat();
    assert!(!hid_advertiser(&report(0x00, &past_end)));
    let ended = [&[0][..], &[3, 0x03, 0x12, 0x18]].concat();
    assert!(!hid_advertiser(&report(0x00, &ended)));
    assert!(!hid_advertiser(&report(0x00, &[3, 0x03, 0x12])));
    assert!(!hid_advertiser(&report(0x00, &[])));
}
