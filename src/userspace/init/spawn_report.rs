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

//! The line init ends its boot spawns with. Every boot check takes
//! "[INIT] Capsules spawned" for a pass, so it is printed only when no capsule
//! failed to spawn; otherwise the line says how many did, and the checks read
//! that line as a failure.

use crate::sys::{boot_log, serial};

pub(super) fn report() {
    let failed = super::capsule_boot::failed();
    if failed == 0 {
        boot_log::ok("INIT", "Capsules spawned");
        return;
    }
    serial::print(b"[INIT] capsule spawn incomplete: ");
    serial::print_dec(u64::from(failed));
    serial::println(b" failed");
    boot_log::error("capsule spawn incomplete; the failures are above");
}
