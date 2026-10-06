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

//! ASPM off while the reader runs (rtsx_disable_aspm). The RTS522A controls
//! it through ASPM_FORCE_CTL (ASPM_MODE_REG); forcing both bits holds the
//! link out of L0s and L1, and Linux waits 10 ms after turning it off. The
//! RTS5227 controls it through the PCIe Link Control register
//! (ASPM_MODE_CFG), which the broker does not let a capsule write; there the
//! firmware's setting stands and the log says so.

use crate::chip::Family;
use crate::clock::sleep_ms;
use crate::error::Result;
use crate::hw::{read_register, write_register};
use crate::log::{emit, Line};
use crate::regs::pm::{ASPM_FORCE_CTL, FORCE_ASPM_CTL0, FORCE_ASPM_CTL1};
use crate::setup::Driver;

const BOTH: u8 = FORCE_ASPM_CTL0 | FORCE_ASPM_CTL1;

pub fn disable_aspm(drv: &Driver) -> Result<()> {
    if drv.family == Family::Rts5227 {
        emit(
            Line::start().text(b"ASPM left as firmware set it (Link Control is not writable here)"),
        );
        return Ok(());
    }
    if read_register(drv.regs, ASPM_FORCE_CTL)? & BOTH == BOTH {
        return Ok(());
    }
    write_register(drv.regs, ASPM_FORCE_CTL, BOTH, BOTH)?;
    sleep_ms(10);
    Ok(())
}
