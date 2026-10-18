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

use crate::constants::{
    LEGACY_WANTED, LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_STATUS, STATUS_ACKNOWLEDGE,
    STATUS_DRIVER, STATUS_FAILED, STATUS_FEATURES_OK,
};
use crate::regs::Regs;
use crate::transport::Transport;

/// Reset the device and take the features this driver handles. The reset
/// comes first and is read back by the clock, so a device left live by a
/// previous instance has dropped its old rings before anything else is
/// written, and before `setup::legacy` lets it master the bus. It used to
/// write zero and go straight on.
pub fn negotiate(regs: Regs) -> Result<u32, &'static str> {
    if !Transport::Legacy(regs).reset() {
        return Err("virtio-net: device did not reset");
    }
    regs.w8(LEG_STATUS, STATUS_ACKNOWLEDGE);
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER);
    let host = regs.r32(LEG_HOST_FEATURES);
    let want = host & LEGACY_WANTED;
    regs.w32(LEG_GUEST_FEATURES, want);
    let s = regs.r8(LEG_STATUS);
    regs.w8(LEG_STATUS, s | STATUS_FEATURES_OK);
    let s2 = regs.r8(LEG_STATUS);
    if s2 & STATUS_FEATURES_OK == 0 {
        regs.w8(LEG_STATUS, s2 | STATUS_FAILED);
        return Err("virtio-net: features-ok rejected");
    }
    Ok(want)
}
