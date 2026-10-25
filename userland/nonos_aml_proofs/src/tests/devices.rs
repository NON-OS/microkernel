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

//! The proofs of what the interpreter reads for devices, each in its own
//! directory beside this file.

#[path = "aei/mod.rs"]
mod aei;
#[path = "apei/mod.rs"]
mod apei;
#[path = "cpu/mod.rs"]
mod cpu;
#[path = "deps/mod.rs"]
mod deps;
#[path = "dpower/mod.rs"]
mod dpower;
#[path = "ged/mod.rs"]
mod ged;
#[path = "idioms/mod.rs"]
mod idioms;
#[path = "legacy/mod.rs"]
mod legacy;
#[path = "motherboard/mod.rs"]
mod motherboard;
#[path = "platform_device/mod.rs"]
mod platform_device;
#[path = "suspend/mod.rs"]
mod suspend;
#[path = "ports/mod.rs"]
mod ports;
#[path = "power_meter/mod.rs"]
mod power_meter;
#[path = "props/mod.rs"]
mod props;
#[path = "shapes/mod.rs"]
mod shapes;
#[path = "serial/mod.rs"]
pub(crate) mod serial;
#[path = "time_alarm/mod.rs"]
mod time_alarm;
#[path = "tpm_ppi/mod.rs"]
mod tpm_ppi;
#[path = "video/mod.rs"]
mod video;
#[path = "wmi/mod.rs"]
mod wmi;
