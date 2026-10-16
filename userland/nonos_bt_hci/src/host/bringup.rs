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

//! Bringing a controller up with standard commands only. Reset first, so
//! nothing an earlier host left set survives; then LE must be supported,
//! or there is nothing for this host to do; then the version, address and
//! ACL buffers are read and the event masks set. A controller that needs
//! vendor setup before it answers these stops at the first that fails.

use super::controller::{bd_addr, buffers, le_buffers, version, Controller};
use super::features::{le_features, le_supported};
use super::masks::{EVENT_MASK, EXTENDED_SCANNER_FILTER, LE_DIRECTED_REPORT, LE_EVENT_MASK};
use super::scan::{ACCEPT_ALL, ACCEPT_UNRESOLVED};
use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;
use nonos_bt_wire::hci::opcode as op;

impl<T: Transport> Host<T> {
    pub fn bring_up(&mut self) -> Result<Controller, HciError> {
        self.command(op::RESET, &[])?;
        if !le_supported(&self.command(op::READ_LOCAL_FEATURES, &[])?)? {
            return Err(HciError::NoLe);
        }
        let v = version(&self.command(op::READ_LOCAL_VERSION, &[])?)?;
        let addr = bd_addr(&self.command(op::READ_BD_ADDR, &[])?)?;
        let le_features = le_features(&self.command(op::LE_READ_LOCAL_FEATURES, &[])?)?;
        let le = le_buffers(&self.command(op::LE_READ_BUFFER_SIZE, &[])?)?;
        let (acl_len, acl_total) = match le {
            Some(pair) => pair,
            None => buffers(&self.command(op::READ_BUFFER_SIZE, &[])?)?,
        };
        if acl_len == 0 || acl_total == 0 {
            return Err(HciError::NoBuffers);
        }
        self.command(op::SET_EVENT_MASK, &EVENT_MASK.to_le_bytes())?;
        // Directed advertising to a private address of ours the controller
        // cannot resolve is heard only under the extended policy.
        let extended = le_features & EXTENDED_SCANNER_FILTER != 0;
        let le_mask = LE_EVENT_MASK | if extended { LE_DIRECTED_REPORT } else { 0 };
        self.command(op::LE_SET_EVENT_MASK, &le_mask.to_le_bytes())?;
        self.scan_filter = if extended { ACCEPT_UNRESOLVED } else { ACCEPT_ALL };
        (self.acl_len, self.acl_total, self.acl_free) = (acl_len, acl_total, acl_total);
        Ok(Controller {
            hci_version: v.0,
            hci_revision: v.1,
            lmp_version: v.2,
            company: v.3,
            lmp_subversion: v.4,
            addr,
            acl_len,
            acl_total,
            le_features,
        })
    }
}
