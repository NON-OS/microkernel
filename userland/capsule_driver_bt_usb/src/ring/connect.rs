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

//! Finding the keyring, from the thread that runs the capsule: the keyring
//! lets only the pid holding driver.bt_usb0 keep its records (lead unit
//! L33), and this capsule has no other thread. That pid is read from the
//! name itself, which needs no CoreExec as getpid would; were the name
//! another's, the kernel's stamp would differ and the keyring refuse.

use super::ipc::Ipc;
use super::keyring::Keyring;
use nonos_libc::mk_service_lookup;

const KEYRING: &[u8] = b"keyring";
const OURS: &[u8] = b"driver.bt_usb0";

/// The keyring, if it is running and driver.bt_usb0 is registered.
pub fn connect() -> Option<Keyring<Ipc>> {
    let (port, _) = lookup(KEYRING)?;
    let (_, pid) = lookup(OURS)?;
    Some(Keyring::new(Ipc { port, seq: 0 }, pid))
}

/// A service's port and the pid that holds its name.
fn lookup(name: &[u8]) -> Option<(u32, u32)> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc >= 0 && port != 0 && pid != 0).then_some((port, pid))
}
