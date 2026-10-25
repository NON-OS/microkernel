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

use super::acked::Acked;
use super::state::{cpu_base, version};
use super::version::GicVersion;
use super::{icc, v2};

/// The highest-priority pending interrupt, now active on this CPU; `None` for
/// a spurious read.
pub fn acknowledge_interrupt() -> Option<Acked> {
    match version()? {
        GicVersion::V3 => icc::acknowledge_interrupt().map(|intid| Acked { intid, raw: intid }),
        GicVersion::V2 => v2::acknowledge(cpu_base()),
    }
}

/// Write `raw`, an acknowledge value, back as this CPU's end of interrupt.
pub(super) fn end(raw: u32) {
    match version() {
        Some(GicVersion::V3) => icc::end_interrupt(raw),
        Some(GicVersion::V2) => v2::end(cpu_base(), raw),
        None => {}
    }
}
