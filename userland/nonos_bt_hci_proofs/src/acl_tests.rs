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

//! ACL data to the controller: an L2CAP PDU cut to the controller's packet
//! length with the LE boundary flags (Vol 4, Part E, 5.4.2), sent only when
//! enough buffers are free, and buffers given back by Number of Completed
//! Packets (Vol 4, Part E, 4.1.1 and 7.7.19).

use crate::sim::config::Config;
use crate::sim::events::completed;
use crate::sim::stack::stack;
use nonos_bt_hci::HciError;
use nonos_bt_wire::WireError;

#[test]
fn a_pdu_goes_as_first_then_continuing_packets() {
    let mut s = stack(Config::default());
    s.bring_up().unwrap();
    let pdu: Vec<u8> = (0..60).collect();
    s.send_pdu(0x0040, &pdu).unwrap();
    let out = &s.transport().pipes().bulk_out;
    assert_eq!(out.len(), 3);
    assert_eq!((&out[0][..4], &out[0][4..]), (&[0x40, 0x00, 27, 0][..], &pdu[..27]));
    assert_eq!((&out[1][..4], &out[1][4..]), (&[0x40, 0x10, 27, 0][..], &pdu[27..54]));
    assert_eq!((&out[2][..4], &out[2][4..]), (&[0x40, 0x10, 6, 0][..], &pdu[54..]));
    assert_eq!(s.acl_free(), 1);
}

#[test]
fn a_pdu_needing_more_buffers_than_free_is_not_started() {
    let mut s = stack(Config::default());
    s.bring_up().unwrap();
    s.send_pdu(0x0040, &[0; 81]).unwrap();
    assert_eq!(s.send_pdu(0x0040, &[0; 28]), Err(HciError::NoCredit));
    assert_eq!(s.transport().pipes().bulk_out.len(), 3);
    s.transport().pipes().send_event(&completed(0x0040, 2));
    assert_eq!(s.next_event(), Ok(None));
    assert_eq!(s.acl_free(), 3);
    s.send_pdu(0x0040, &[0; 28]).unwrap();
    s.transport().pipes().send_event(&completed(0x0040, 9));
    assert_eq!(s.next_event(), Ok(None));
    assert_eq!(s.acl_free(), 4);
}

#[test]
fn no_acl_before_bring_up_and_no_empty_pdu() {
    let mut s = stack(Config::default());
    assert_eq!(s.send_pdu(0x0040, &[1, 0, 4, 0, 9]), Err(HciError::NoBuffers));
    s.bring_up().unwrap();
    assert_eq!(s.send_pdu(0x0040, &[]), Err(HciError::Wire(WireError::Short)));
}
