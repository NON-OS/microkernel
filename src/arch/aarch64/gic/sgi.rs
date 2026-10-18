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

use super::state::{dist_base, version};
use super::v2;
use super::version::GicVersion;

/// SGI `intid` to the CPU with packed affinity `target`.
pub fn send_sgi(target: u32, intid: u32) -> Result<(), ()> {
    match version() {
        Some(GicVersion::V3) => super::icc::send_sgi(target, intid),
        Some(GicVersion::V2) => v2::send(dist_base(), target, intid),
        None => Err(()),
    }
}

pub fn send_sgi_all_others(intid: u32) -> Result<(), ()> {
    match version() {
        Some(GicVersion::V3) => super::icc::send_sgi_all_others(intid),
        Some(GicVersion::V2) => v2::send_all_others(dist_base(), intid),
        None => Err(()),
    }
}
