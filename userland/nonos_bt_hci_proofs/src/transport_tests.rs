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

//! The USB transport itself: a command goes as the class request of
//! Vol 4, Part B, 2.2 with the packet untouched as its data stage, and a
//! bulk OUT the device takes only part of is an error.

use crate::sim::config::Config;
use crate::sim::stack::stack;
use nonos_bt_hci::usb::{Setup, COMMAND};
use nonos_bt_hci::{HciError, Transport};

#[test]
fn a_command_is_a_class_request_to_the_device_with_the_packet_as_data() {
    assert_eq!(COMMAND, Setup { request_type: 0x20, request: 0, value: 0, index: 0 });
    let mut s = stack(Config::default());
    s.transport().send_command(&[0x03, 0x0c, 0x00]).unwrap();
    let (setup, data, _) = &s.transport().pipes().commands[0];
    assert_eq!((*setup, data.as_slice()), (COMMAND, &[0x03, 0x0c, 0x00][..]));
}

#[test]
fn acl_goes_out_on_bulk_and_a_short_take_is_refused() {
    let mut s = stack(Config::default());
    s.transport().send_acl(&[0x40, 0x00, 1, 0, 9]).unwrap();
    assert_eq!(s.transport().pipes().bulk_out, vec![vec![0x40, 0x00, 1, 0, 9]]);
    s.transport().pipes().bulk_take = Some(3);
    assert_eq!(s.transport().send_acl(&[0x40, 0x00, 1, 0, 9]), Err(HciError::Transport(-5)));
}

#[test]
fn acl_in_is_joined_across_bulk_transfers() {
    let mut s = stack(Config::default());
    let t = s.transport();
    t.pipes().bulk_in.extend([vec![0x40, 0x20, 6, 0, 2, 0], vec![4, 0, 1, 2]]);
    assert_eq!(t.poll_acl(), Ok(None));
    assert_eq!(t.poll_acl(), Ok(Some(vec![0x40, 0x20, 6, 0, 2, 0, 4, 0, 1, 2])));
    assert_eq!(t.poll_acl(), Ok(None));
}
