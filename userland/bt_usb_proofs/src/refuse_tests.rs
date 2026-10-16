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

//! Links this host gives up on, each for its own reason, with the HID
//! service never read on a link that was not encrypted (the air fails the
//! proof if it is), and the link then ended with Disconnect.

use crate::air::Act;
use crate::fixture::{air, keyboard, link, with, HID_ADV};
use crate::hogp_sim::keyboard_mouse;
use crate::link::{hang_up, End};
use crate::smp_sim::Peer;
use nonos_bt_hci::HciError;
use nonos_bt_smp::method::IoCap;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn numeric_comparison_is_declined_and_pairing_ends() {
    // Both sides have a display and a yes or no: Numeric Comparison
    // (Vol 3, Part H, 2.3.5.1), which the user declines here.
    let peer = Peer::new(IoCap::DisplayYesNo, true);
    let (mut host, ours, mut env) = with(peer, keyboard_mouse(), &HID_ADV);
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("pairing failed"));
    assert!(s.bond.is_none());
    assert_eq!(air(&mut host).ltk, None, "no key was ever used");
}

#[test]
fn a_refused_key_ends_the_link_and_hang_up_closes_it() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).refuse_encryption = true;
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("encryption failed"));
    assert_eq!(hang_up(&mut host, s.handle), Ok(()));
    let last = air(&mut host).commands.last().unwrap().clone();
    assert_eq!(last[..2], op::DISCONNECT.0.to_le_bytes());
    assert_eq!(last[3..], [0x40, 0x00, 0x13], "handle, Remote User Terminated");
    assert!(env.input.is_empty());
}

#[test]
fn a_silent_peripheral_runs_out_the_att_timer() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).hid.silent = true;
    let (end, s) = link(&mut host, &ours, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("HID service unusable"));
    assert!(s.bond.is_some(), "pairing finished first");
    assert!(env.now >= 30_000, "gave up early, at {} ms", env.now);
}

/// The controller must be brought up again (Vol 4, Part E, 7.7.16): the
/// link and the controller's run end together.
#[test]
fn a_hardware_error_ends_the_link_and_the_controller_run() {
    let (mut host, ours, mut env) = keyboard();
    air(&mut host).script.push_back(Act::Event(vec![0x10, 1, 0x00]));
    assert_eq!(link(&mut host, &ours, &mut env).err(), Some(HciError::Reset));
}
