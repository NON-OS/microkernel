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

//! the rows of the map: MkPioGrant's window (A10 gaps 3 and 5) and
//! MkIrqWait's bound (gap 1). MkPciConfigWrite's command rule is in
//! kernel_proofs::pci_command.

use super::irq_bound::{wait_ms, DEFAULT_WAIT_MS, MAX_WAIT_MS};
use super::pio_window::{window, Refused, RESERVED};

#[test]
fn a_pio_window_never_includes_the_machines_own_ports() {
    for &(lo, hi) in &RESERVED {
        assert_eq!(window(u64::from(lo), 1), Err(Refused::Reserved), "{lo:#x}");
        assert_eq!(window(u64::from(hi), 1), Err(Refused::Reserved), "{hi:#x}");
        assert_eq!(window(u64::from(lo) - 4, 8), Err(Refused::Reserved), "across {lo:#x}");
    }
    assert_eq!(window(0xC00, 0x400), Err(Refused::Reserved), "a BAR over 0xCF8");
    assert_eq!(window(0x60, 5), Ok((0x60, 5)), "the PS/2 controller's own ports");
    assert_eq!(window(0xC000, 0x40), Ok((0xC000, 0x40)), "a NIC's BAR");
    assert_eq!(window(0x22, 0x1E), Ok((0x22, 0x1E)), "between the PIC and the PIT");
}

#[test]
fn a_pio_window_counts_right_or_is_refused() {
    assert_eq!(window(0, 0x10000), Err(Refused::Overflow), "64 KiB: not zero ports");
    assert_eq!(window(0xFFF0, 0x20), Err(Refused::Overflow), "past 0xFFFF");
    assert_eq!(window(0x1_0000, 1), Err(Refused::Overflow), "a base past the space");
    assert_eq!(window(0x1000, 0), Err(Refused::Overflow), "no ports");
    assert_eq!(window(0xFFF0, 0x10), Ok((0xFFF0, 0x10)), "up to the last port");
}

#[test]
fn every_irq_wait_has_a_deadline() {
    assert_eq!(wait_ms(0), DEFAULT_WAIT_MS);
    assert_eq!(wait_ms(250), 250);
    assert_eq!(wait_ms(MAX_WAIT_MS), MAX_WAIT_MS);
    assert_eq!(wait_ms(MAX_WAIT_MS + 1), MAX_WAIT_MS);
    assert_eq!(wait_ms(u64::MAX), MAX_WAIT_MS, "forever is cut");
}
