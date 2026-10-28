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

//! Batteries and AC adapters, and a laptop's battery path whole: its tables
//! through the interpreter, capsule_acpi's fold into the kernel's word, the
//! kernel's report, and the bar's and About's words, each mounted by path.

#[path = "../../../../capsule_about/src/about/data/battery.rs"]
mod about_battery;
#[path = "../../../../capsule_desktop_shell/src/state/indicators/battery_text/mod.rs"]
mod battery_text;
mod info_tests;
mod laptop;
mod laptop_read;
mod laptop_regs;
mod laptop_tests;
#[path = "../../../../../src/hardware/broker/acpi/power_report.rs"]
mod power_report;
mod report_tests;
#[path = "../../../../capsule_acpi/src/power/state.rs"]
mod state;
mod status_tests;
#[path = "../../../../capsule_acpi/src/power/word.rs"]
mod word;
