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

//! Handing the kernel each processor's ceiling when it changes. Nothing in
//! the kernel acts on it until track A's frequency control reads it; the
//! critical trip still powers the machine off whatever the ceiling is.

use super::ceilings::{changes, record};
use super::state::Cooling;
use crate::sys::acpi_thermal_limit::acpi_thermal_limit;
use crate::sys::log::Line;
use alloc::vec::Vec;

pub fn send_limits(c: &mut Cooling) {
    let zones: Vec<(&[u64], u8)> =
        c.zones.iter().filter_map(|z| z.passive.as_ref()).map(|p| (&p.uids[..], p.limit)).collect();
    for (uid, limit) in changes(&zones, &c.sent) {
        let l = Line::new(b"thermal: cpu ").dec(uid).str(b" ceiling ").dec(u64::from(limit));
        // A refusal is said once and not retried until the limit moves again.
        let l =
            if acpi_thermal_limit(uid, limit).is_ok() { l.str(b"%") } else { l.str(b"% refused") };
        l.end();
        record(&mut c.sent, uid, limit);
    }
}
