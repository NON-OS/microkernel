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

use super::surface::Surface;
use crate::dispi::Span;
use crate::handles::BrokerHandles;
use crate::regs::Regs;

/// The adapter, driven. Fields drop in order: the surface is unmapped, then
/// the handles write the firmware's DISPI state back, give up both mappings
/// and release the claim last.
pub struct Driver {
    pub surface: Surface,
    pub _handles: BrokerHandles,
    pub regs: Regs,
    /// The mode's pixels in the framebuffer the adapter scans out.
    pub fb: Span,
    pub width: u16,
    pub height: u16,
    pub stride: u32,
    /// The compositor the primary surface was last handed to; 0 until it
    /// asks.
    pub owner_pid: u32,
}
