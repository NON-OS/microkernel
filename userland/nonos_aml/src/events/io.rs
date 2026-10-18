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

//! The port reads and writes the event layer makes, through the capsule's
//! MkAcpiPort (or, in the proofs, a model of the registers).

pub trait EventIo {
    /// One read of `width` bytes (1 or 2); None if refused.
    fn read(&mut self, port: u16, width: u8) -> Option<u16>;
    /// One write; false if refused.
    fn write(&mut self, port: u16, width: u8, value: u16) -> bool;
    /// Wait `ms` milliseconds, for the ACPI mode switch.
    fn sleep(&mut self, ms: u64);
    /// Milliseconds on a clock that only moves forward, for the storm
    /// bounds. An io with no clock reads as one instant, so every run it
    /// makes counts in one window: storms are caught sooner, never later.
    fn now_ms(&mut self) -> u64 {
        0
    }
}
