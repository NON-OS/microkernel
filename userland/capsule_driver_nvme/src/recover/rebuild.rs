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

use crate::admin::hmb::{ends_attempt, ENABLE_TIMEOUT_MS};
use crate::admin::set_features::FID_HOST_MEMORY_BUFFER;
use crate::admin::AdminQueue;
use crate::error::NvmeResult;
use crate::nvm::IoQueue;
use crate::setup::Device;

/// Reset the controller and bring its queues back in the memory they had.
/// The disable drops every command the controller held, the stuck one with
/// them, and once it completes the controller touches none of that memory:
/// only then do the rings start over. The host memory buffer, which the
/// disable took back, is given again before the I/O queues exist, as at
/// bring-up; Number of Queues is not asked again, as its value holds across
/// a reset. Each step is bounded: CAP.TO for the disable and enable, the
/// admin deadlines for the commands.
pub fn rebuild(
    device: &mut Device,
    admin: &mut AdminQueue,
    io: &mut IoQueue,
    hmb: Option<[u32; 5]>,
) -> NvmeResult<()> {
    device.disable()?;
    admin.restart(device.regs);
    io.restart();
    device.enable()?;
    let (regs, stride) = (device.regs, device.info.doorbell_stride());
    if let Some(dw) = hmb {
        let what = "give the host memory buffer back after a reset";
        match admin.set_features(regs, stride, FID_HOST_MEMORY_BUFFER, dw, what, ENABLE_TIMEOUT_MS)
        {
            Err(e) if ends_attempt(e) => return Err(e),
            // Refused: the drive is served without it, as the spec allows.
            _ => {}
        }
    }
    io.create(admin, regs, stride)
}
