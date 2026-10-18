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

//! One controller's run: bring-up, then links one after another, and the
//! controller told to forget them before it is given back.

use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::{HciError, Host};

use super::env::CapsuleEnv;
use super::links::links;
use super::say::say;
use crate::link::{forget, Private};
use crate::usb::{Bound, XhciPipes};
use nonos_bt_bond::LocalIrk;

/// ACL packets are taken up to this many data octets until bring-up has
/// read the controller's own length: the LE minimum (Vol 4, Part E, 7.8.2).
const ACL_FIRST: u16 = 27;

/// Bring-up, then links one after another; the error that ended it, once
/// the controller has been told to forget them.
pub fn controller(b: Bound, irk: &LocalIrk, env: &mut CapsuleEnv) -> HciError {
    let mut host = Host::new(UsbHci::new(b.pipes, &b.iface, ACL_FIRST));
    let why = run_on(&mut host, irk, env);
    forget(&mut host, &why);
    why
}

fn run_on(host: &mut Host<UsbHci<XhciPipes>>, irk: &LocalIrk, env: &mut CapsuleEnv) -> HciError {
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
    links(host, Private::new(irk.clone(), c.addr), env)
}
