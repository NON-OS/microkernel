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

use super::cpu_interface::CpuInterface;
use super::encode::iar_intid;
use crate::arch::aarch64::gic::acked::Acked;

pub(in crate::arch::aarch64::gic) fn acknowledge(cpu: u64) -> Option<Acked> {
    let raw = CpuInterface::new(cpu).acknowledge();
    Some(Acked { intid: iar_intid(raw)?, raw })
}

/// Write back the whole acknowledge value: for an SGI it names the sender,
/// and an EOI without it finishes nothing.
pub(in crate::arch::aarch64::gic) fn end(cpu: u64, raw: u32) {
    CpuInterface::new(cpu).end(raw);
}
