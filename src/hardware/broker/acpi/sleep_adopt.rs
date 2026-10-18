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

//! The interpreter's \_S5, kept beside the constant reader's for the
//! kernel's soft-off. The constant reader at boot finds \_S5 only when
//! firmware wrote it as a plain package; a laptop that computes it in a
//! method left soft-off unavailable, and the machine parked instead of
//! powering off. The constant reader's pair stands when there is one
//! (`s5_choice`); shutdown still reads the registry, never AML.

use super::kernel::platform::fw::aml::sleep_obj::SleepPackage;
use super::kernel::platform::fw::parser::state::TABLES;
use super::s5_choice::{choose, posture, Source};

/// Keeps the interpreter's pair and returns the constant reader's; None
/// when the ACPI tables were never read, and there is nowhere to put it.
pub fn adopt_s5(a: u8, b: u8) -> Option<Option<(u8, u8)>> {
    let scan = {
        let mut tables = TABLES.write();
        let reg = tables.as_mut()?;
        reg.data.s5_interp = Some(SleepPackage { slp_typ_a: a, slp_typ_b: b });
        reg.data.s5.map(|p| (p.slp_typ_a, p.slp_typ_b))
    };
    super::report::posture();
    Some(scan)
}

/// Where soft-off's pair comes from now; None when there is none.
pub fn soft_off() -> Option<Source> {
    let tables = TABLES.read();
    let d = &tables.as_ref()?.data;
    let pair = |p: Option<SleepPackage>| p.map(|p| (p.slp_typ_a, p.slp_typ_b));
    choose(pair(d.s5), pair(d.s5_interp)).map(|(_, from)| from)
}

/// The posture line's word for soft-off.
pub fn soft_off_posture() -> &'static [u8] {
    posture(soft_off())
}
