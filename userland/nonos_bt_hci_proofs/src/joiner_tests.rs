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

//! Whole packets out of USB transfers, cut by the length in each packet's
//! own header (Vol 4, Part E, 5.4.2 and 5.4.4).

use nonos_bt_hci::usb::Joiner;
use nonos_bt_hci::HciError;
use nonos_bt_wire::WireError;

#[test]
fn an_event_split_over_three_transfers_comes_out_whole() {
    let mut event = vec![0x3e, 38];
    event.extend((0..38).map(|i| i as u8));
    let mut j = Joiner::events();
    for chunk in event.chunks(16) {
        assert_eq!(j.take(), Ok(None));
        j.push(chunk);
    }
    assert_eq!(j.take(), Ok(Some(event)));
    assert_eq!(j.held(), 0);
}

#[test]
fn two_events_in_one_transfer_come_out_in_order() {
    let mut j = Joiner::events();
    j.push(&[0x0e, 4, 1, 0x03, 0x0c, 0x00, 0x13, 5, 1, 0x40, 0x00]);
    assert_eq!(j.take(), Ok(Some(vec![0x0e, 4, 1, 0x03, 0x0c, 0x00])));
    assert_eq!(j.take(), Ok(None));
    j.push(&[0x01, 0x00]);
    assert_eq!(j.take(), Ok(Some(vec![0x13, 5, 1, 0x40, 0x00, 0x01, 0x00])));
}

#[test]
fn acl_joins_by_its_two_octet_length_and_refuses_past_the_bound() {
    let mut j = Joiner::acl(27);
    j.push(&[0x40, 0x20, 5, 0]);
    j.push(&[1, 2, 3, 4, 5]);
    assert_eq!(j.take(), Ok(Some(vec![0x40, 0x20, 5, 0, 1, 2, 3, 4, 5])));
    j.push(&[0x40, 0x20, 28, 0, 9]);
    assert_eq!(j.take(), Err(HciError::Wire(WireError::TooLong)));
    assert_eq!(j.held(), 0);
}
