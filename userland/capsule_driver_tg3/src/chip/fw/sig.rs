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

//! The driver-state words around a reset.

use crate::chip::sram;
use crate::constants::sram::*;
use crate::regs::Regs;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Init,
    Shutdown,
}

pub fn sig_pre_reset(regs: &Regs, asf: bool, kind: Kind) {
    sram::write(regs, NIC_SRAM_FIRMWARE_MBOX, NIC_SRAM_FIRMWARE_MBOX_MAGIC1);
    sig_legacy(regs, asf, kind);
}

/// tg3_write_sig_legacy; on a 5750_PLUS part tg3_write_sig_pre_reset
/// writes the same state word.
pub fn sig_legacy(regs: &Regs, asf: bool, kind: Kind) {
    if asf {
        let state = if kind == Kind::Init { DRV_STATE_START } else { DRV_STATE_UNLOAD };
        sram::write(regs, NIC_SRAM_FW_DRV_STATE_MBOX, state);
    }
}

pub fn sig_post_reset(regs: &Regs, asf: bool, kind: Kind) {
    if asf {
        let done = if kind == Kind::Init { DRV_STATE_START_DONE } else { DRV_STATE_UNLOAD_DONE };
        sram::write(regs, NIC_SRAM_FW_DRV_STATE_MBOX, done);
    }
}
