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

//! How a capsule's interrupt line is sensed. Pure, so the rule is held on the
//! host (kernel_proofs ioapic_line_mode).
//!
//! An ACPI platform device's own `_CRS` interrupt descriptor says it first
//! (ACPI 6.5, 6.4.3.6; F 51 carries it in the record): a GPIO controller on
//! GSI 7 or 14 asks for level, active-low, and sensed as an ISA edge it fired
//! once and never again (L43). Then an MADT override. Without either, GSIs 0
//! to 15 are the ISA lines, edge-triggered and active-high (ACPI 6.5,
//! 5.2.12.5). A line above them reaches a capsule only as a PCI INTx or an
//! ACPI device interrupt, and PCI INTx# is level-sensitive and active-low
//! (PCI Local Bus 3.0, 2.2.6). Sensed as an ISA edge, such a line fires only
//! when the device lets go of it, so a driver waiting for its first interrupt
//! waits forever.

pub(crate) const ISA_LINES: u32 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LineMode {
    pub(crate) level: bool,
    pub(crate) active_low: bool,
}

/// `described` is the device's own (level, active-low) when its record
/// carries one; `iso` the override's when the MADT has one.
pub(crate) const fn line_mode(
    gsi: u32,
    iso: Option<(bool, bool)>,
    described: Option<(bool, bool)>,
) -> LineMode {
    if let Some((level, active_low)) = described {
        return LineMode { level, active_low };
    }
    match iso {
        Some((level, active_low)) => LineMode { level, active_low },
        None if gsi >= ISA_LINES => LineMode { level: true, active_low: true },
        None => LineMode { level: false, active_low: false },
    }
}
