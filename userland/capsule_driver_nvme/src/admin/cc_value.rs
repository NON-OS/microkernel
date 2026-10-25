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

//! The Controller Configuration written to enable the controller, from CAP
//! alone (NVMe 2.0 sections 3.1.3.1 and 3.1.3.5). Enable used to write only
//! EN and the queue entry sizes: CC.MPS was never set, a controller whose
//! MPSMIN was not 4 KiB was refused without the reason, and CAP.CSS was
//! never read, so a controller with no NVM command set was enabled and
//! sent NVM reads it could not run.

use crate::constants::{
    cap_mpsmax_shift, cap_mpsmin_shift, cap_nvm_supported, CC_AMS_RR, CC_CSS_NVM, CC_EN,
    CC_IOCQES_16, CC_IOSQES_64, CC_MPS_SHIFT, DRIVER_PAGE_SHIFT,
};
use crate::error::{NvmeError, NvmeResult};

/// CC for a controller with capabilities `cap`: the NVM command set, round
/// robin, 64-byte submission and 16-byte completion entries, and MPS at
/// MPSMIN, the smallest page the controller takes.
pub const fn cc_value(cap: u64) -> NvmeResult<u32> {
    if !cap_nvm_supported(cap) {
        return Err(NvmeError::NoNvmCommandSet);
    }
    let (min, max) = (cap_mpsmin_shift(cap), cap_mpsmax_shift(cap));
    // A largest page below the smallest is no page range at all.
    if max < min {
        return Err(NvmeError::UnsupportedController);
    }
    if min > DRIVER_PAGE_SHIFT {
        return Err(NvmeError::UnsupportedPageSize);
    }
    let mps = (min - 12) as u32;
    Ok(CC_EN | CC_CSS_NVM | (mps << CC_MPS_SHIFT) | CC_AMS_RR | CC_IOSQES_64 | CC_IOCQES_16)
}
