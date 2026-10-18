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

//! Probing the remapping units at boot and saying on the console what was
//! found. Nothing here enables translation, and firmware's protection is
//! kept: it is let go after bring-up, or here under the development opt-in
//! alone (iommu::handover). Whether DMA ends up confined is bring-up's
//! verdict to print, so this module never states a posture too early.

use super::super::probe::{probe_all, unit_count};
use super::{describe, failure, state};
use crate::sys::serial::{self, Line};

/// Probe and report. Called once, after ACPI parsing has published the DRHD
/// bases and the MMIO mapper can hand out a register window.
pub fn init() {
    let count = unit_count();
    if count == 0 {
        serial::println(b"[VT-D] no remapping units in DMAR; DMA is unrestricted");
        return;
    }

    let units = match probe_all() {
        Ok(units) => units,
        Err(e) => {
            let mut line = Line::new();
            line.str(b"[VT-D] probe failed (").str(failure::reason(e));
            line.str(b"); DMA is unrestricted").end();
            return;
        }
    };
    for info in units.iter() {
        describe::unit(count, info);
    }
    super::firmware::release_at_probe(&units);
    let shared = match state::merge(&units) {
        Ok(shared) => shared,
        Err(_) => {
            serial::println(b"[VT-D] units share no paging depth; DMA is unrestricted");
            return;
        }
    };
    state::record(units, shared);
}
