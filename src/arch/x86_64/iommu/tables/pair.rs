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
//! 128-bit entries whose present bit sits in the low quadword: root and
//! context entries, interrupt remapping entries.

use core::sync::atomic::{compiler_fence, Ordering};

use super::out_of_frame::OutOfFrame;
use super::view::Frame;

/// The high quadword, then the low one that makes the entry live, so a unit
/// that reads it present reads all of it. Volatile stores keep their order;
/// the fence is there so no later edit of this function can lose it.
pub fn set_pair(frame: Frame, slot: usize, low: u64, high: u64) -> Result<(), OutOfFrame> {
    frame.write(slot + 1, high)?;
    compiler_fence(Ordering::Release);
    frame.write(slot, low)
}

/// The low quadword first, so the unit stops honouring the entry before its
/// other half is forgotten.
pub fn clear_pair(frame: Frame, slot: usize) -> Result<(), OutOfFrame> {
    frame.write(slot, 0)?;
    compiler_fence(Ordering::Release);
    frame.write(slot + 1, 0)
}
