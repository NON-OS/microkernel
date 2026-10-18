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

//! Firmware's protection let go at probe: under the development opt-in
//! alone, and never against the DMAR platform opt-in (iommu::handover).
//! Otherwise bring-up lets the protected regions go once its tables are live.

use super::super::probe::UnitInfo;
use crate::arch::x86_64::acpi::parser::other::dmar_platform_opt_in;
use crate::arch::x86_64::iommu::handover::{vtd, Moment};
use crate::arch::x86_64::iommu::unit::enable::release_from_firmware;
use crate::memory::iommu::dev_unconfined;
use crate::sys::serial::{self, Line};

pub(super) fn release_at_probe(units: &[UnitInfo]) {
    let enforce = cfg!(feature = "nonos-iommu-enforce");
    if vtd(enforce, dev_unconfined::holds(), dmar_platform_opt_in()) != Moment::AtProbe {
        return;
    }
    for info in units {
        // SAFETY: eK@nonos.systems - the development opt-in alone, which
        // accepts unrestricted DMA; the posture line names it.
        match unsafe { release_from_firmware(&info.unit) } {
            Ok(r) if r.protected_regions || r.translation => {
                let mut line = Line::new();
                line.str(b"[VT-D] firmware left");
                if r.protected_regions {
                    line.str(b" protected memory regions");
                }
                if r.translation {
                    line.str(b" translation");
                }
                line.str(b" on; turned off").end();
            }
            Ok(_) => {}
            Err(_) => serial::println(b"[VT-D] firmware protection would not turn off"),
        }
    }
}
