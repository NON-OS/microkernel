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

use super::line_mode::{line_mode, LineMode, ISA_LINES};

const ISA: LineMode = LineMode { level: false, active_low: false };
const PCI: LineMode = LineMode { level: true, active_low: true };

#[test]
fn isa_lines_without_an_override_are_edge_and_active_high() {
    for gsi in 0..ISA_LINES {
        assert_eq!(line_mode(gsi, None), ISA, "gsi {gsi}");
    }
}

// The lines a PCH routes PIRQA..H to (16 to 23 on Intel), and the wider
// IO-APICs of AMD and server boards, carry PCI INTx#: level, active-low.
#[test]
fn lines_above_the_isa_range_are_sensed_as_pci_intx() {
    for gsi in [16, 17, 19, 23, 40, 119, 255] {
        assert_eq!(line_mode(gsi, None), PCI, "gsi {gsi}");
    }
}

#[test]
fn an_override_is_taken_as_written() {
    // The ACPI SCI, IRQ 9 on GSI 9, level and active-low on most firmware.
    assert_eq!(line_mode(9, Some((true, true))), PCI);
    // The timer moved from IRQ 0 to GSI 2, edge and active-high.
    assert_eq!(line_mode(2, Some((false, false))), ISA);
    // An override that names a line above 15 as an edge is believed.
    assert_eq!(line_mode(20, Some((false, false))), ISA);
    assert_eq!(line_mode(20, Some((true, false))), LineMode { level: true, active_low: false });
}
