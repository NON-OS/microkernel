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

use super::encode::{sgir_all_others, sgir_to};
use super::masks;
use crate::arch::aarch64::gic::distributor::GicDistributor;

/// SGI `intid` to the CPU with packed affinity `target`. Refused for a CPU
/// whose interface number is not known yet, rather than sent to nobody.
pub(in crate::arch::aarch64::gic) fn send(dist: u64, target: u32, intid: u32) -> Result<(), ()> {
    let mask = masks::of_affinity(target).ok_or(())?;
    let value = sgir_to(mask, intid).ok_or(())?;
    GicDistributor::new(dist).write_sgir(value);
    Ok(())
}

pub(in crate::arch::aarch64::gic) fn send_all_others(dist: u64, intid: u32) -> Result<(), ()> {
    let value = sgir_all_others(intid).ok_or(())?;
    GicDistributor::new(dist).write_sgir(value);
    Ok(())
}
