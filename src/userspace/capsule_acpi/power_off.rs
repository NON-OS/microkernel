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

//! `power.off`, the ACPI capsule's second name (F-88): the shell's request
//! for the wiped power-off, under a name of its own so HELD admits the
//! shell alone. Its messages land in the same inbox as `acpi`'s, and the
//! capsule checks the sender again, so neither name widens the other.

use crate::capabilities::Capability;
use crate::services::registry::{register_endpoint, required_caps};

const NAME: &str = "power.off";
/// As capsule_acpi's Capsule.mk declares it.
const PORT: u32 = 4532;

pub(super) fn register(pid: u32) {
    let caps = required_caps(NAME, Capability::IPC.bit());
    // A failure costs only the shell's request; the capsule runs on.
    if register_endpoint(NAME, PORT, pid, caps).is_err() {
        crate::sys::serial::println(b"[ACPI] power.off endpoint not registered");
    }
}
