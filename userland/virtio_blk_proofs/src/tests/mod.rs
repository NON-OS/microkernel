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

//! The driver against windows, a ring and buffers in host memory, a device
//! that answers late, short or out of turn, a scripted interrupt line and
//! clock; and the orders in its source that only a boot runs.

mod bench;
mod device;
mod frames;
mod legacy_dev;
mod legacy_ports;
mod legacy_serve;
mod model;
mod rig;
mod script;
mod served;
mod source;

mod abi_tests;
mod bringup_tests;
mod chain_tests;
mod clock_tests;
mod data_tests;
mod discover_tests;
mod fields_tests;
mod irq_tests;
mod legacy_len_tests;
mod legacy_tests;
mod order_tests;
mod parse_bounds_tests;
mod parse_read_tests;
mod parse_write_tests;
mod recover_tests;
mod regs_tests;
mod reset_tests;
mod rollback_tests;
mod rules_tests;
mod size_tests;
mod slice_tests;
mod status_tests;
mod streak_span_tests;
mod streak_tests;
mod teardown_tests;
mod used_tests;
mod wait_tests;
mod wire_tests;
mod wrap_tests;
