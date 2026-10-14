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

//! The service lookup that says whether a shield capsule answers.

use nonos_libc::mk_service_lookup;

use super::probe::{Shield, SHIELD_SERVICE};

/// Look for the shield capsule.
///
/// Cheap enough to call on a screen change and not on every paint: a service
/// lookup is an IPC round trip, and a wallet that probes sixty times a second
/// is a wallet that spends its time asking questions it already answered.
pub fn probe() -> Shield {
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    let rc = mk_service_lookup(
        SHIELD_SERVICE.as_ptr(),
        SHIELD_SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    // Anything other than a successful lookup naming a live pid is absence.
    // Failing closed here is the whole point: an unexpected return must never
    // read as availability.
    if rc >= 0 && pid != 0 {
        Shield::Present
    } else {
        Shield::Absent
    }
}
