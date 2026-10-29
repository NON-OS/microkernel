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

//! Where each part's gate stands in the capsule's own source, and the call
//! it guards, so a gate taken away or moved off its call fails the proofs.

use super::rule::Part::{self, *};

const LATE: &str = include_str!("../../../capsule_acpi/src/boot/late.rs");
const FIRST: &str = include_str!("../../../capsule_acpi/src/boot/first.rs");
const UP: &str = include_str!("../../../capsule_acpi/src/boot/bringup.rs");
const S5: &str = include_str!("../../../capsule_acpi/src/sleep/start.rs");
const BTP: &str = include_str!("../../../capsule_acpi/src/power/alarm.rs");
const OFF: &str = include_str!("../../../capsule_acpi/src/power/off/serve.rs");
const BUTTON: &str = include_str!("../../../capsule_acpi/src/power/off/button.rs");
const NOTIFY: &str = include_str!("../../../capsule_acpi/src/server/notify.rs");
pub const REPLY: &str = include_str!("../../../capsule_acpi/src/server/reply.rs");

/// A gate the power-only build turns off, as `release::on(Part::X)` reads.
const NOT: &str = "if!crate::release::on(crate::release::Part::";

/// The part, its file, the gate (whitespace taken out), and the call the
/// gate's block must hold before it closes; "" when the gate returns.
pub const SITES: [(Part, &str, &str, &str); 21] = [
    (Pic, UP, "ifon(Part::Pic){", "set_pic("),
    (Osc, UP, "ifon(Part::Osc){", "negotiate_all("),
    (Sleep, S5, "Sleep){return;", ""),
    (Reserved, FIRST, "ifon(Part::Reserved){", "reserved::publish("),
    (Hidden, FIRST, "!on(Part::Hidden)||", "reveal_hidden("),
    (Hidden, LATE, "ifon(Part::Hidden){", "reveal_hidden("),
    (Hotplug, FIRST, "ifon(Part::Hotplug){", "hotplug::learn("),
    (Hotplug, LATE, "ifon(Part::Hotplug){", "native::find("),
    (Hotplug, NOTIFY, "on(Part::Hotplug)&&", "hotplug::notified("),
    (Platform, FIRST, "ifon(Part::Platform){", "platform::publish("),
    (BatteryTrip, BTP, "BatteryTrip){return;", ""),
    (PowerOff, OFF, "PowerOff){", "return"),
    (FixedButton, BUTTON, "fixed_button(fadt)&&taken_here()", ""),
    (FixedButton, BUTTON, "if!fixed_button(fadt)||!taken_here(){", "return;"),
    (Cooling, LATE, "ifon(Part::Cooling){", "cooling::find("),
    (Cpu, LATE, "ifon(Part::Cpu){", "cpu::find("),
    (Backlight, LATE, "ifon(Part::Backlight){", "backlight::find("),
    (HwErrors, LATE, "ifon(Part::HwErrors){", "apei::start("),
    (Ports, LATE, "ifon(Part::Ports){", "ports::publish("),
    (TypeC, LATE, "ifon(Part::TypeC){", "typec::find("),
    (DevicePower, NOTIFY, "on(Part::DevicePower)&&", "dpower::woken("),
];

/// The whole gate as the file spells it: `NOT` before the short ones.
pub fn gate(file: &str, gate: &str) -> String {
    let needs_not = [S5, BTP, OFF].contains(&file);
    if needs_not {
        format!("{NOT}{gate}")
    } else {
        gate.to_string()
    }
}
