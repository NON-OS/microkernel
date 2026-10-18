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

use super::fail::fail;
use crate::constants::{LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_STATUS};
use crate::constants::{STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_FEATURES_OK};
use crate::regs::Regs;
use crate::transport::Transport;

/// Reset, read back by the clock, then ACKNOWLEDGE, DRIVER, no feature bits
/// (virtio-rng needs none, and claiming one the device does not offer is a
/// protocol violation), then FEATURES_OK, read back. A device left live by
/// a previous instance has dropped its ring before anything else is written
/// and before `setup::legacy` lets it master the bus; the reset used to be
/// written and not waited for.
pub fn negotiate(regs: Regs) -> Result<(), &'static str> {
    if !Transport::Legacy(regs).reset() {
        return Err("virtio-rng: device did not reset");
    }
    regs.w8(LEG_STATUS, STATUS_ACKNOWLEDGE);
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER);
    let _host = regs.r32(LEG_HOST_FEATURES);
    regs.w32(LEG_GUEST_FEATURES, 0);
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FEATURES_OK);
    if regs.r8(LEG_STATUS) & STATUS_FEATURES_OK == 0 {
        return fail(regs, "virtio: features-ok rejected");
    }
    Ok(())
}
