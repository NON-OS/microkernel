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

//! The authorization gate's ops: the held devices listed, a held one
//! answered, and the policy set. Each is the kernel's alone (pid 0): the
//! shell's path to them is the kernel's to give, and no capsule may bind a
//! device by answering for it.

use super::mux::Mux;
use super::mux_held::held;
use crate::protocol::{Request, E_INVAL, OP_USB_AUTHORIZE, OP_USB_HELD, OP_USB_POLICY};
use crate::server::error::reply_with_status;
use crate::server::reply::sender;
use crate::slots::Policy;

const E_PERM: i32 = -1;

impl Mux {
    /// True when the request was one of these and is answered.
    pub(super) fn auth_ops(&mut self, req: &Request, body: &[u8], tx: &mut [u8]) -> bool {
        if !matches!(req.op, OP_USB_HELD | OP_USB_AUTHORIZE | OP_USB_POLICY) {
            return false;
        }
        let auth = &mut self.share.auth;
        let status = match (req.op, body) {
            _ if sender() != 0 => E_PERM,
            (OP_USB_HELD, []) => return held(auth.held(), req, tx),
            (OP_USB_AUTHORIZE, &[slot, allow @ (0 | 1), t0, t1, t2, t3]) => {
                let ticket = u32::from_le_bytes([t0, t1, t2, t3]);
                if auth.answer(slot, ticket, allow == 1) {
                    0
                } else {
                    E_INVAL
                }
            }
            (OP_USB_POLICY, &[p]) => {
                match [Policy::Allow, Policy::Ask, Policy::Deny].get(p as usize) {
                    Some(&policy) => {
                        auth.policy = policy;
                        0
                    }
                    None => E_INVAL,
                }
            }
            _ => E_INVAL,
        };
        reply_with_status(tx, req, status);
        true
    }
}
