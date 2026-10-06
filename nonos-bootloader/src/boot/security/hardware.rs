// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use uefi::prelude::*;

use crate::display::{show_error_screen, update_stage, StageStatus, STAGE_SECURITY};
use crate::log::logger::log_error;
use crate::security::{
    check_minimum_requirements, detect_hardware_capabilities, HardwareCapabilities,
};

use super::super::util::fatal_reset;

pub fn verify_hardware_requirements(st: &mut SystemTable<Boot>, gop: bool) -> HardwareCapabilities {
    let hw_caps = detect_hardware_capabilities();
    let hw_reqs = check_minimum_requirements(&hw_caps);
    if !hw_reqs.passed {
        let why: &[u8] = if !hw_reqs.nx_bit {
            b"This CPU has no NX bit: NONOS keeps writable memory from running, and cannot here"
        } else {
            b"This CPU addresses under 36 bits of physical memory"
        };
        log_error("security", "Hardware requirements not met");
        update_stage(STAGE_SECURITY, StageStatus::Failed);
        if gop {
            show_error_screen(why);
        }
        fatal_reset(st, "Hardware requirements not met");
    }
    hw_caps
}
