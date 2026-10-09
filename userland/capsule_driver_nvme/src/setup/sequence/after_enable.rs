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

use super::served::served;
use crate::admin::{AdminQueue, ControllerIdentity, NamespaceIdentity, SmartHealth};
use crate::controller::ControllerInfo;
use crate::discover::Found;
use crate::error::NvmeResult;
use crate::nvm::IoQueue;
use crate::regs::Regs;
use crate::setup::hmb::{self, Hmb};

pub(super) type Brought =
    (ControllerIdentity, NamespaceIdentity, SmartHealth, Option<IoQueue>, Option<Hmb>);

pub(super) fn after_enable(
    dev: Found,
    claim_epoch: u64,
    regs: Regs,
    info: ControllerInfo,
    admin: &mut AdminQueue,
) -> NvmeResult<Brought> {
    let stride = info.doorbell_stride();
    let identity = {
        let data = admin.identify_controller(regs, stride)?;
        ControllerIdentity::parse(data)
    };
    // Both before any I/O queue exists, as Linux and the spec order them,
    // and only until an attempt that asked for them has failed.
    let given = if hmb::extras() {
        hmb::number_of_queues(admin, regs, stride)?;
        hmb::host_memory(admin, regs, info, dev.device_id, claim_epoch, &identity)?
    } else {
        None
    };
    match served(dev, claim_epoch, regs, info, admin, &identity, given.is_some()) {
        Ok((namespace, health, io)) => Ok((identity, namespace, health, io, given)),
        Err(e) => {
            // Take the buffer back from the controller before its memory goes.
            if let Some(memory) = given {
                memory.release(regs, info);
            }
            Err(e)
        }
    }
}
