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

//! The capsule's Intel firmware loader, its files unchanged, against a
//! modelled Intel controller (`model`): a bootloader that takes Secure Send
//! on bulk OUT, answers on the interrupt endpoint or on bulk IN, reports
//! its Secure Send Result, and boots into firmware with its bootup event.

#[path = "../../../capsule_driver_bt_usb/src/vendor/intel/mod.rs"]
pub mod loader;

mod model;
mod model_cmds;
mod model_pipes;
mod model_send;
mod rig;

mod carried_tests;
mod ddc_tests;
mod flow_tests;
mod legacy_tests;
mod name_tests;
mod refuse_tests;
mod send_tests;
mod timeout_tests;
mod version_tests;
