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

//! What the interpreter asks of the capsule around it.

use crate::object::Object;

pub trait Host {
    /// Sleep (section 19.6.128): give up the processor for `ms` milliseconds.
    fn sleep(&mut self, ms: u64);
    /// Stall (section 19.6.129): busy-wait `us` microseconds.
    fn stall(&mut self, us: u64);
    /// Timer (section 19.6.136): a monotonic count of 100 ns ticks.
    fn timer(&mut self) -> u64;
    /// A Store to the Debug object.
    fn debug(&mut self, value: &Object);
    /// Fatal's operands, reported before the evaluation stops.
    fn fatal(&mut self, kind: u8, code: u32, arg: u64);
}
