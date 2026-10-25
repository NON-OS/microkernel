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

//! The SLP_EN rule's place in the kernel, read from the source: every port
//! write the ACPI capsule asks for is checked before the `out`, PM1 control
//! is checked only for SLP_EN, and a driver's PIO grant still keeps off all
//! four power registers.

const ACCESS: &str = include_str!("../../../../src/hardware/broker/acpi/port_access.rs");
const REGS: &str = include_str!("../../../../src/hardware/broker/acpi/kernel/power_regs.rs");
const MACHINE: &str = include_str!("../../../../src/hardware/broker/acpi/kernel/machine.rs");
const PIO: &str = include_str!("../../../../src/hardware/broker/pio/bar_window.rs");
const POWER_PORTS: &str = include_str!("../../../../src/hardware/broker/pio/power_ports.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing: {needle}"))
}

#[test]
fn a_port_write_is_checked_for_slp_en_before_it_is_made() {
    let write = at(ACCESS, "pub fn port_write(");
    let checked = at(ACCESS, "if super::kernel::power_regs::sets_sleep_enable(at, width, value) {");
    let refused = at(ACCESS, "return Err(RegionError::SleepEnable);");
    let out = at(&ACCESS[write..], "outb(p, value as u8)") + write;
    assert!(write < checked && checked < refused && refused < out);
}

#[test]
fn the_capsule_keeps_off_the_reset_register_and_drivers_keep_off_all_four() {
    assert!(MACHINE.contains("super::power_regs::reset_touched(SPACE_IO, base, end)"));
    assert!(REGS.contains("power_touches(power_regs()[3].iter(), space, base, end)"));
    assert!(REGS.contains("if f.has_reset_reg() { reg(f.reset_reg) } else { None },"));
    assert!(POWER_PORTS.contains("if power_regs::touches(SPACE_IO, lo, hi) {"));
    assert!(PIO.contains("clear(window(bar.base, bar.size))"));
}
