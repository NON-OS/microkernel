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

use crate::admin::hmb::ends_attempt;
use crate::admin::{AdminQueue, ControllerIdentity, NamespaceIdentity, SmartHealth};
use crate::controller::ControllerInfo;
use crate::discover::Found;
use crate::error::{NvmeError, NvmeResult};
use crate::nvm::{IoQueue, NamespaceGeometry};
use crate::regs::Regs;
use crate::setup::{namespace, say};

/// The namespace served, its health, and its I/O queue when it can be served.
pub(super) fn served(
    dev: Found,
    claim_epoch: u64,
    regs: Regs,
    info: ControllerInfo,
    admin: &mut AdminQueue,
    identity: &ControllerIdentity,
    hmb_given: bool,
) -> NvmeResult<(NamespaceIdentity, SmartHealth, Option<IoQueue>)> {
    let stride = info.doorbell_stride();
    let namespace = match namespace::served_nsid(admin, regs, info, identity)? {
        Some(nsid) => {
            let data = admin.identify_namespace(regs, stride, nsid)?;
            NamespaceIdentity::parse(nsid, data)
        }
        None => NamespaceIdentity::absent(),
    };
    // The SMART / health log is diagnostics: a drive that refuses it, or
    // whose answer never comes, still stores data. Its failure is said by
    // the admin wait and the snapshot reads zero. But no answer right after
    // the host memory buffer was taken is a controller the buffer stopped
    // (the HP's PM991 did, 6 Oct): the attempt fails, and the next one runs
    // the drive without the buffer.
    let health = match admin.smart_health(regs, stride) {
        Ok(data) => SmartHealth::parse(data),
        Err(NvmeError::ClockFailed) => return Err(NvmeError::ClockFailed),
        Err(e) if hmb_given && ends_attempt(e) => return Err(e),
        Err(_) => SmartHealth::unread(),
    };
    let io = match NamespaceGeometry::check(identity, &namespace) {
        Ok(geometry) => {
            say::geometry(&namespace, &geometry);
            // The namespace can be served, so a failure to build its queue is
            // the attempt's: the admin wait has said which command failed and
            // how, and the schedule tries again.
            Some(crate::nvm::bring_up(dev.device_id, claim_epoch, regs, stride, admin, geometry)?)
        }
        Err(why) => {
            say::refused(&namespace, why);
            None
        }
    };
    Ok((namespace, health, io))
}
