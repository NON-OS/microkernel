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

use super::version::GicVersion;
use crate::sys::serial::Line;

/// One line naming the GIC chosen and where, so a boot log shows which
/// backend ran before anything it does can go wrong.
pub(super) fn chosen(version: GicVersion, dist: u64, cpu: u64, redist: u64) {
    let mut line = Line::new();
    match version {
        GicVersion::V2 => {
            line.str(b"[GIC] v2, distributor ").hex(dist).str(b", cpu interface ").hex(cpu)
        }
        GicVersion::V3 => {
            line.str(b"[GIC] v3, distributor ").hex(dist).str(b", redistributors ").hex(redist)
        }
    };
    line.end();
}
