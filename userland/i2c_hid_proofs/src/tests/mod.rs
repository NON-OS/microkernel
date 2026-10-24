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

//! The touchpad driver from service lookup to posted pointer events.

mod acpi_tests;
mod bind_tests;
mod bounds_more_tests;
mod bounds_size_tests;
mod bounds_tests;
mod budget_tests;
mod bystander_tests;
mod composite_tests;
mod configure_tests;
mod descs;
mod digitizers;
mod doorbell_tests;
mod driven;
mod feature_edge_tests;
mod fixture;
mod frames;
mod fuzz_tests;
mod gate_open_tests;
mod gate_tests;
mod gesture_multi_tests;
mod gesture_tests;
mod hostile_desc_tests;
mod hostile_gen;
mod hostile_tests;
mod keyboard_tests;
mod layout_fuzz_tests;
mod layout_tests;
mod line_tests;
mod long_report_tests;
mod long_tests;
mod mouse_app_tests;
mod multitouch_tests;
mod no_scan_tests;
mod pace_tests;
mod pace_unit_tests;
mod park_tests;
mod poll_scroll_tests;
mod poll_tests;
mod quiet_tests;
mod range_tests;
mod real_pad_tests;
mod recorder;
mod recover_tests;
mod recovery;
mod repeat_tests;
mod reset_more_tests;
mod reset_tests;
mod retain_tests;
mod rng;
mod screen_tests;
mod service;
mod setup_tests;
mod stick_tests;
mod torn_tests;
mod vendor_tests;
mod wake_tests;
mod walker_tests;
mod window_tests;
mod wired;
