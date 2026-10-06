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

use crate::constants::*;
use crate::regs::Regs;

// Intel LPSS wrapper bring-up, as Linux intel_lpss_init_dev does it: put the
// function in reset, release the function and both integrated DMA resets,
// then program the remap address (the window's physical address, low dword
// then high) so the integrated DMA sees the window where it really is, above
// 4 GiB included. Firmware frequently leaves the resets released already;
// asserting first makes the state the same whatever it left. This has to run
// before the first DesignWare register access, and only on an LPSS function:
// a platform controller has no private block at these offsets.
pub(super) fn lpss_init(regs: Regs, base: u64) {
    regs.write32(LPSS_PRIV_RESETS, 0);
    regs.write32(LPSS_PRIV_RESETS, LPSS_PRIV_RESETS_DEASSERT);
    regs.write32(LPSS_PRIV_REMAP_LO, base as u32);
    regs.write32(LPSS_PRIV_REMAP_HI, (base >> 32) as u32);
}
