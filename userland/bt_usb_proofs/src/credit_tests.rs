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

//! Controller buffers (Vol 4, Part E, 4.1.1): a controller with only
//! three LE buffers, which frees them late, cannot take pairing's public
//! key (three packets of 27 octets) while the Pairing Request's is still
//! out, so the PDU waits whole for Number of Completed Packets and is then
//! sent; none is dropped or sent in part.

use std::cell::Cell;
use std::rc::Rc;

use crate::air::{Act, Air};
use crate::env::TestEnv;
use crate::fixture::{link, HID_ADV};
use crate::hci_sim::stack::IFACE;
use crate::hogp_sim::{keyboard_mouse, Peripheral};
use crate::link::End;
use crate::smp_sim::Peer;
use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::Host;
use nonos_bt_smp::method::IoCap;

#[test]
fn pairing_waits_for_buffers_and_still_finishes() {
    let shown = Rc::new(Cell::new(None));
    let peer = Peer::new(IoCap::KeyboardOnly, true);
    let mut air = Air::new(&HID_ADV, peer, Peripheral::new(keyboard_mouse()), shown.clone());
    air.cfg.le_buffers = (27, 3);
    air.lazy_credits = true;
    air.script.push_back(Act::Drop(0x13));
    let mut host = Host::new(UsbHci::new(air, &IFACE, 27));
    let c = host.bring_up().unwrap();
    host.transport().set_acl_max(c.acl_len);
    let mut env = TestEnv::new(shown);
    let (end, s) = link(&mut host, &c.addr, &mut env).unwrap();
    assert_eq!(end, End::Lost(0x13));
    assert!(s.bond.is_some(), "pairing did not finish");
    let ready = env.lines.iter().any(|l| l.starts_with("HID ready"));
    assert!(ready, "{:?}", env.lines);
}
