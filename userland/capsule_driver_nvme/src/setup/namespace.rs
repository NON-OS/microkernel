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

use super::say;
use crate::admin::{
    first_active_nsid, lists_active_namespaces, AdminQueue, ControllerIdentity, FALLBACK_NSID,
};
use crate::controller::ControllerInfo;
use crate::error::{NvmeError, NvmeResult};
use crate::regs::Regs;

/// The NSID this controller is served through: the first active one the
/// controller lists (Identify CNS 02h), or NSID 1 on a controller that
/// cannot list them. None for a controller with no namespace, or none
/// active, which is served for identify and health alone.
pub fn served_nsid(
    admin: &mut AdminQueue,
    regs: Regs,
    info: ControllerInfo,
    identity: &ControllerIdentity,
) -> NvmeResult<Option<u32>> {
    if identity.namespace_count == 0 {
        say::no_namespace(identity.namespace_count);
        return Ok(None);
    }
    if !lists_active_namespaces(info.version) {
        return Ok(Some(FALLBACK_NSID));
    }
    match admin.identify_active_namespaces(regs, info.doorbell_stride()) {
        Ok(page) => {
            let nsid = first_active_nsid(page, identity.namespace_count);
            if nsid.is_none() {
                say::no_namespace(identity.namespace_count);
            }
            Ok(nsid)
        }
        // Refused: a controller older than its VS says, or one that does not
        // implement the list. The admin wait has said how.
        Err(NvmeError::AdminCommandFailed) => {
            say::list_refused();
            Ok(Some(FALLBACK_NSID))
        }
        Err(e) => Err(e),
    }
}
