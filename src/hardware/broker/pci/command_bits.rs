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

//! The PCI Command bits a driver may change, and what each form of the
//! write leaves in the register. Pure, so kernel_proofs holds it by path.

pub(crate) const MEMORY_SPACE: u16 = 1 << 1;
pub(crate) const BUS_MASTER: u16 = 1 << 2;
pub(crate) const INTX_DISABLE: u16 = 1 << 10;

/// Memory Space, Bus Master and Interrupt Disable; `command.rs` says why.
pub(crate) const WRITABLE: u16 = MEMORY_SPACE | BUS_MASTER | INTX_DISABLE;

/// The plain form: a value holding only writable bits is ORed onto the
/// register, so a driver turns its bits on without knowing what firmware
/// left; any other value is taken whole. `None` when the result changes a
/// bit outside `WRITABLE`.
pub(crate) const fn plain(new: u16, current: u16) -> Option<u16> {
    let desired = if new & !WRITABLE == 0 { current | new } else { new };
    exact(desired, current)
}

/// The masked form: `desired` is what the register becomes, refused when
/// it differs from `current` outside `WRITABLE`.
pub(crate) const fn exact(desired: u16, current: u16) -> Option<u16> {
    if (desired ^ current) & !WRITABLE != 0 {
        return None;
    }
    Some(desired)
}
