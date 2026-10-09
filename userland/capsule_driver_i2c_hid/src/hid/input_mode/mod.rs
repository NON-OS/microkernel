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

//! Drive a touchpad's reporting configuration to the state this device class
//! actually works in. Three feature fields govern whether a Precision
//! Touchpad reports at all and in which shape: Input Mode (0x0D:0x52,
//! 3 = the touch collection, the only collection PTP-only pads generate
//! reports on), the Surface switch (0x0D:0x57, 0 = no motion reports, the
//! pad looks dead), and the Button switch (0x0D:0x58, 0 = no button
//! reports). A previous session, another OS on a warm reboot, or a buggy
//! host can leave any of them off. Every field is located from the report
//! descriptor and applied read-modify-write at bit granularity; when the
//! device refuses GET_REPORT, a descriptor-accurate report is constructed
//! from the parsed field positions instead.

mod command;
mod configure;
mod fallback;
mod get;
mod one_report;
mod set;
mod set_bits;

pub use configure::configure_reporting;
