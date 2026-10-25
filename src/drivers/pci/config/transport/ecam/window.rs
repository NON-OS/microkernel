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

//! One ECAM window: a run of buses in one segment whose config space is
//! mapped, one megabyte a bus, at `va`. Pure, so the host proofs hold it.

const BUS_SHIFT: u64 = 20;
const DEVICE_SHIFT: u64 = 15;
const FUNCTION_SHIFT: u64 = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EcamWindow {
    pub segment: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    /// The mapping of `start_bus`, not of bus 0. A producer whose table gives
    /// the bus-0 base, as MCFG does, applies the offset once itself.
    pub va: u64,
}

impl EcamWindow {
    pub const fn covers(&self, segment: u16, bus: u8) -> bool {
        segment == self.segment && bus >= self.start_bus && bus <= self.end_bus
    }

    /// Zero for an empty window, one whose end bus is below its start.
    pub const fn bytes(&self) -> u64 {
        if self.is_empty() {
            return 0;
        }
        ((self.end_bus - self.start_bus) as u64 + 1) << BUS_SHIFT
    }

    pub const fn is_empty(&self) -> bool {
        self.end_bus < self.start_bus || self.va == 0
    }

    /// Whether the two windows claim a bus in common.
    pub const fn overlaps(&self, other: &EcamWindow) -> bool {
        self.segment == other.segment
            && self.start_bus <= other.end_bus
            && other.start_bus <= self.end_bus
    }

    /// The address of one register, or `None` for a device above 31, a
    /// function above 7, an offset past 0xFFF or a bus outside the window.
    /// Nothing out of range is masked into range: that would reach another
    /// function's registers.
    pub const fn locate(&self, bus: u8, device: u8, function: u8, offset: u16) -> Option<u64> {
        if device > 31 || function > 7 || offset > 0xFFF {
            return None;
        }
        if self.is_empty() || bus < self.start_bus || bus > self.end_bus {
            return None;
        }
        let displacement = (((bus - self.start_bus) as u64) << BUS_SHIFT)
            | ((device as u64) << DEVICE_SHIFT)
            | ((function as u64) << FUNCTION_SHIFT)
            | offset as u64;
        self.va.checked_add(displacement)
    }
}
