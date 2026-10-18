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
    LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_STATUS, STATUS_ACKNOWLEDGE, STATUS_DRIVER,
    STATUS_FAILED, STATUS_FEATURES_OK,
};
use crate::regs::Regs;
use crate::transport::Transport;

/// The one device feature taken. DISCARD, WRITE_ZEROES, MQ, RO, BLK_SIZE,
/// TOPOLOGY and config-change handling are not built (plan/57 names them
/// their own unit), and a bit the driver does not handle is never claimed.
const VIRTIO_BLK_F_FLUSH: u32 = 1 << 9;

/// Reset, seen done, then ACKNOWLEDGE, DRIVER, FLUSH when offered, and
/// FEATURES_OK read back. A device a dead predecessor left running may take
/// a while to stop its queue: until its status reads back zero it may still
/// reach that ring, so nothing goes on, Bus Master least of all, before
/// then (`setup::legacy`); one that never stops is refused by name.
pub fn negotiate(regs: Regs) -> Result<(), &'static str> {
    if !Transport::Legacy(regs).reset() {
        return Err("virtio-blk: reset never read back");
    }
    regs.w8(LEG_STATUS, STATUS_ACKNOWLEDGE);
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER);
    regs.w32(LEG_GUEST_FEATURES, regs.r32(LEG_HOST_FEATURES) & VIRTIO_BLK_F_FLUSH);
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FEATURES_OK);
    if regs.r8(LEG_STATUS) & STATUS_FEATURES_OK == 0 {
        return fail(regs, "virtio-blk: features-ok rejected");
    }
    Ok(())
}

/// Tell the device the driver gave up on it, and say why.
pub(super) fn fail<T>(regs: Regs, why: &'static str) -> Result<T, &'static str> {
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
    Err(why)
}
