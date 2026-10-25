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

//! What the boot pass leaves in a function's Command register (L34). Pure,
//! so kernel_proofs checks it by path.

pub(crate) const IO_SPACE: u16 = 1 << 0;
pub(crate) const MEMORY_SPACE: u16 = 1 << 1;

/// While a BAR is sized: the decoders off, or for the width of the probe the
/// function claims a window that is not its own.
pub(crate) const fn while_sizing(found: u16) -> u16 {
    found & !(IO_SPACE | MEMORY_SPACE)
}

/// Afterwards: the decoders on, so a driver finds its BARs answering, and
/// Bus Master as firmware left it. This pass used to turn it on for every
/// endpoint, so a device no driver had claimed could reach memory by DMA
/// before anything confined it. A driver turns it on for its own device.
pub(crate) const fn after_assign(found: u16) -> u16 {
    found | IO_SPACE | MEMORY_SPACE
}
