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

//! The capsule's life: find a controller, run it until it fails, give its
//! slot back, and look again.

use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::{HciError, Host};
use nonos_usbnet::xhci::disable_slot;

use super::bonds;
use super::env::CapsuleEnv;
use super::finder::Finder;
use super::links::links;
use super::say::{reason, say};
use crate::link::Private;
use crate::usb::Bound;
use nonos_bt_bond::LocalIrk;

/// Between two looks at the ports while no controller is bound.
const SEARCH_MS: u64 = 1_000;
/// ACL packets are taken up to this many data octets until bring-up has
/// read the controller's own length: the LE minimum (Vol 4, Part E, 7.8.2).
const ACL_FIRST: u16 = 27;

pub fn run() -> ! {
    let (mut env, irk) = bonds::environment();
    let mut finder = Finder::new();
    say(&[b"started, looking for a controller"]);
    loop {
        let Some((bound, port)) = finder.pass(&env.desk) else {
            env.desk.wait(SEARCH_MS);
            continue;
        };
        let (xhci, slot) = (bound.pipes.bus.xhci, bound.pipes.bus.slot);
        let why = controller(bound, &irk, &mut env);
        env.desk.board().controller = false;
        say(&[b"controller given back: ", reason(&why)]);
        disable_slot(xhci, slot);
        finder.lost(port);
    }
}

/// Bring-up, then links one after another; the error that ended it.
fn controller(b: Bound, irk: &LocalIrk, env: &mut CapsuleEnv) -> HciError {
    let mut host = Host::new(UsbHci::new(b.pipes, &b.iface, ACL_FIRST));
    let c = match host.bring_up() {
        Ok(c) => c,
        Err(e) => return e,
    };
    host.transport().set_acl_max(c.acl_len);
    // An all-zero address is a controller still waiting for its vendor's
    // setup, which this host does not do (letter E-12); it also could not
    // stand for this host in pairing (Vol 3, Part H, 2.2.7).
    if c.addr.0 == [0; 6] {
        return HciError::NotBluetooth;
    }
    say(&[b"controller up"]);
    env.desk.board().controller = true;
    links(&mut host, Private::new(irk.clone(), c.addr), env)
}
