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

//! A request that binds a device (slots/binds.rs) weighed against its
//! standing (slots/authorize.rs). SET_CONFIGURATION is weighed against the
//! configuration it sets, a pipe or hub against the current one; one not
//! yet judged is read here, through the same path as a class driver's
//! read, and judged. Under Allow nothing is read and nothing changes.

use nonos_libc::mk_debug;

use super::mux::Mux;
use super::mux_config::config_at;
use crate::protocol::Request;
use crate::server::error::reply_with_status;
use crate::slots::binds::{binds, set_configuration};
use crate::slots::Standing;

const HELD: &[u8] = b"xhci: a device is held for the owner's answer (OP_USB_HELD)";
/// Configurations looked through for the one a SET_CONFIGURATION names.
const CONFIGS: u8 = 8;

impl Mux {
    /// True when the bind was refused and answered.
    pub(super) fn bind_refused(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
        let Some(&slot) = body.first().filter(|_| binds(req.op, body)) else { return false };
        let set = set_configuration(req.op, body);
        let auth = |m: &mut Mux| match set {
            Some(v) => m.share.auth.admit_set(slot, v),
            None => m.share.auth.admit(slot),
        };
        if auth(self).is_none() {
            let (value, config) = match set {
                Some(v) => (
                    v,
                    (0..CONFIGS)
                        .map_while(|i| config_at(self, req, slot, i, tx))
                        .find(|c| c.get(5) == Some(&v)),
                ),
                None => {
                    let c = config_at(self, req, slot, 0, tx);
                    (c.as_ref().and_then(|c| c.get(5).copied()).unwrap_or(0), c)
                }
            };
            if let Standing::Held { .. } = self.share.auth.judge(slot, value, config.as_deref()) {
                let _ = mk_debug(HELD.as_ptr(), HELD.len());
            }
        }
        match auth(self) {
            Some(Err(e)) => {
                reply_with_status(tx, req, e);
                true
            }
            _ => false,
        }
    }
}
