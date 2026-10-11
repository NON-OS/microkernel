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

//! The slot's register window.

/// The slot's register window. Offsets are from the slot base (SDHCI 3.0,
/// 2.1). Implementations access the device with volatile, naturally sized
/// loads and stores.
pub trait Mmio {
    fn r8(&self, off: u32) -> u8;
    fn r16(&self, off: u32) -> u16;
    fn r32(&self, off: u32) -> u32;
    fn w8(&self, off: u32, v: u8);
    fn w16(&self, off: u32, v: u16);
    fn w32(&self, off: u32, v: u32);
}
