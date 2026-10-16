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


use super::query::capabilities;
use super::unconfined::unconfined_grants;
use crate::sys::serial::Line;

/*
 * The vendor is "none" on a machine with neither DMAR remapping units nor an
 * IVRS table, so every line has the same shape for the boot matrix to read.
 */
pub(super) fn posture_line() {
    let caps = capabilities();
    Line::new()
        .str(b"[IOMMU] ")
        .str(caps.vendor.name())
        .str(b" present, enforcing=")
        .str(flag(caps.enforcing))
        .str(b", unconfined grants=")
        .dec(u64::from(unconfined_grants()))
        .str(super::dev_unconfined::posture_tail())
        .end();
}

pub(super) const fn flag(on: bool) -> &'static [u8] {
    if on {
        b"1"
    } else {
        b"0"
    }
}
