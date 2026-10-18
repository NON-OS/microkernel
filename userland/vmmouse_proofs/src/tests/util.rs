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

//! A host with a backdoor, and the driver up on it.

use nonos_libc::{backdoor_calls, idled, posted};

use crate::model::{Host, VMWARE};
use crate::post::Pointer;
use crate::serve::Driver;
use crate::setup;

pub fn host() -> Host {
    let h = Host::new();
    h.install(VMWARE);
    h
}

/// The pointer enabled, with every record so far cleared.
pub fn up() -> (Driver, Host) {
    let h = host();
    setup::enable().expect("pointer up");
    let _ = (backdoor_calls(), posted(), idled());
    (Driver { pointer: Pointer::default(), lost: false }, h)
}
