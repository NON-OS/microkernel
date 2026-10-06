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

//! A function's BARs, reduced to what capability validation checks. The
//! driver fills these from the broker's device record.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarKind {
    Absent,
    Mmio,
    Io,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BarInfo {
    pub kind: BarKind,
    pub size: u64,
}

impl BarInfo {
    pub const ABSENT: Self = Self { kind: BarKind::Absent, size: 0 };

    pub const fn mmio(size: u64) -> Self {
        if size == 0 {
            return Self::ABSENT;
        }
        Self { kind: BarKind::Mmio, size }
    }

    pub const fn io(size: u64) -> Self {
        if size == 0 {
            return Self::ABSENT;
        }
        Self { kind: BarKind::Io, size }
    }

    pub const fn is_mmio(self) -> bool {
        matches!(self.kind, BarKind::Mmio) && self.size != 0
    }

    pub const fn is_io(self) -> bool {
        matches!(self.kind, BarKind::Io) && self.size != 0
    }
}

/// Indexed by BAR number, the way the device record lists them. The upper
/// half of a 64-bit BAR is listed absent.
pub type Bars = [BarInfo; 6];

/// A legacy (or transitional) virtio-pci function keeps its registers in an
/// I/O BAR; a modern-only one has none.
pub fn has_io_bar(bars: &Bars) -> bool {
    bars.iter().any(|b| b.is_io())
}
