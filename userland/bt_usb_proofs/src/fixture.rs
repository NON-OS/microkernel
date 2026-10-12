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

//! A link from scan to its end, as the capsule's `run::links` runs one,
//! over the air with a HID device in it.

use std::cell::Cell;
use std::rc::Rc;

use crate::air::Air;
use crate::env::TestEnv;
use crate::hci_sim::stack::IFACE;
use crate::hogp_sim::{keyboard_mouse, Db, Peripheral};
use crate::link::{find_hid, serve, End, Session};
use crate::smp_sim::Peer;
use nonos_bt_hci::host::{Connect, DEFAULT_LINK, DEFAULT_SCAN};
use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::{HciError, Host};
use nonos_bt_smp::method::IoCap;
use nonos_bt_wire::addr::BdAddr;

pub type Stack = Host<UsbHci<Air>>;

/// Flags, then the HID service in a complete 16-bit UUID list.
pub const HID_ADV: [u8; 7] = [2, 0x01, 0x06, 3, 0x03, 0x12, 0x18];

/// A keyboard with a pointing stick that pairs by passkey entry: it has
/// keys and no display (Vol 3, Part H, 2.3.5.1).
pub fn keyboard() -> (Stack, BdAddr, TestEnv) {
    with(Peer::new(IoCap::KeyboardOnly, true), keyboard_mouse(), &HID_ADV)
}

pub fn with(smp: Peer, db: Db, adv: &[u8]) -> (Stack, BdAddr, TestEnv) {
    let shown = Rc::new(Cell::new(None));
    let air = Air::new(adv, smp, Peripheral::new(db), shown.clone());
    let mut host = Host::new(UsbHci::new(air, &IFACE, 27));
    let c = host.bring_up().unwrap();
    host.transport().set_acl_max(c.acl_len);
    (host, c.addr, TestEnv::new(shown))
}

pub fn air(host: &mut Stack) -> &mut Air {
    host.transport().pipes()
}

/// Find, connect, pair and serve; how the link ended and its session.
pub fn link(
    host: &mut Stack,
    ours: &BdAddr,
    env: &mut TestEnv,
) -> Result<(End, Session), HciError> {
    let peer = find_hid(host, env)?.expect("no HID device heard");
    let c =
        Connect { scan: DEFAULT_SCAN, peer_type: peer.kind, peer: peer.addr, link: DEFAULT_LINK };
    let link = host.connect(&c)?;
    let mut s = Session::open(&link, ours, env).expect("pairing did not start");
    let end = serve(host, &mut s, env)?;
    Ok((end, s))
}
