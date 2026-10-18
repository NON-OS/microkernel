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

use super::super::error::PsciError;
use super::super::function::{
    PSCI_SYSTEM_OFF, PSCI_SYSTEM_RESET, PSCI_SYSTEM_RESET2_64, PSCI_SYSTEM_SUSPEND_64,
};
use super::super::raw::{psci_call0, psci_call2};

pub fn system_off() -> ! {
    stranded(b"SYSTEM_OFF", psci_call0(PSCI_SYSTEM_OFF))
}

pub fn system_reset() -> ! {
    stranded(b"SYSTEM_RESET", psci_call0(PSCI_SYSTEM_RESET))
}

pub fn system_reset2(reset_type: u32, cookie: u64) -> ! {
    stranded(b"SYSTEM_RESET2", psci_call2(PSCI_SYSTEM_RESET2_64, reset_type as u64, cookie))
}

pub fn system_suspend(entry_point: u64, context_id: u64) -> Result<(), PsciError> {
    PsciError::from_ret(psci_call2(PSCI_SYSTEM_SUSPEND_64, entry_point, context_id) as i32)
}

/// These calls return only when they did not happen. Say why on the console
/// before parking, or a machine asked to go away just stops answering.
fn stranded(call: &[u8], ret: i64) -> ! {
    let mut line = crate::sys::serial::Line::new();
    line.str(b"[PSCI] ").str(call);
    match PsciError::from_ret(ret as i32) {
        _ if !super::super::is_present() => line.str(b": the device tree names no PSCI"),
        Err(e) => line.str(b" refused: ").str(e.as_str().as_bytes()),
        Ok(()) => line.str(b" returned"),
    };
    line.str(b"; CPU parked").end();
    wait_forever()
}

fn wait_forever() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfi", options(nomem, nostack));
        }
    }
}
