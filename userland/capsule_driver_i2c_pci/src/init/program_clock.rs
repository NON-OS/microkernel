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

use super::scl;
use crate::constants::*;
use crate::regs::Regs;

/// IC_COMP_VERSION from which IC_SDA_HOLD exists ("1.11a").
const SDA_HOLD_MIN_VERSION: u32 = 0x3131_312A;

// Writes the standard and fast SCL count pairs plus the SDA hold time. Only
// valid while IC_ENABLE is 0; these registers are read-only once enabled.
// IC_SDA_HOLD exists from core version 1.11a; on an older core the write
// would land on an offset the core does not decode, so it is skipped as
// Linux skips it.
pub(super) fn program_clock(regs: Regs, clock_hz: u32) {
    let ss = scl::standard(clock_hz);
    let fs = scl::fast(clock_hz);
    regs.write32(IC_SS_SCL_HCNT, ss.hcnt);
    regs.write32(IC_SS_SCL_LCNT, ss.lcnt);
    regs.write32(IC_FS_SCL_HCNT, fs.hcnt);
    regs.write32(IC_FS_SCL_LCNT, fs.lcnt);
    regs.write32(IC_FS_SPKLEN, scl::fs_spklen(clock_hz));
    if regs.read32(IC_COMP_VERSION) >= SDA_HOLD_MIN_VERSION {
        regs.write32(IC_SDA_HOLD, scl::sda_hold(clock_hz));
    }
}
