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

//! `power.off`, capsule_power's second name: the shell's request for the
//! wiped power-off, under a name of its own so HELD admits the shell alone
//! (src/services/registry/held_table.rs). Registered here at spawn, never at
//! run time, so no other capsule can take the name first. Its messages land
//! in the same inbox as `power`'s, and the capsule checks the sender again.

use crate::capabilities::Capability;
use crate::services::registry::{register_endpoint, required_caps};

const NAME: &str = "power.off";
/// As capsule_power's Capsule.mk declares it.
const PORT: u32 = 4446;

pub(super) fn register(pid: u32) {
    let caps = required_caps(NAME, Capability::IPC.bit());
    // A failure costs the shell's power-off; the capsule runs on, and the
    // shell says the power-off is not available.
    if register_endpoint(NAME, PORT, pid, caps).is_err() {
        crate::sys::serial::println(b"[POWER] power.off endpoint not registered");
    }
}
