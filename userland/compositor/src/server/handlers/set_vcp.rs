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

//! NCMP 0x0015 SET_VCP: one of a monitor's own controls set, the input
//! source among them, from desktop_shell alone (E_PERM otherwise), which
//! sends it only when the person chose it. Body: panel u32, code u32 (at
//! most 0xFF), value u32 (at most 0xFFFF); any other is E_INVAL. Carried to
//! the driver as F 12's SET_VCP, which holds the code to its allowlist and
//! the value to what the monitor listed; its refusal comes back as E_PERM
//! or E_INVAL, a driver that serves no controls as E_NOENT, and no answer
//! or any other refusal as E_IO.

use crate::display::brightness_rule::OP_SET_VCP;
use crate::protocol::{Request, E_INVAL, E_IO, E_NOENT, E_PERM, SET_VCP_REQ_LEN};
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if !super::shell_only::from_shell(sender) {
        return respond::status(sender, req, E_PERM, tx);
    }
    let (Some(panel), Some(code), Some(value)) =
        (super::u32_at(body, 0), super::u32_at(body, 4), super::u32_at(body, 8))
    else {
        return respond::status(sender, req, E_INVAL, tx);
    };
    if body.len() != SET_VCP_REQ_LEN || code > 0xFF || value > 0xFFFF {
        return respond::status(sender, req, E_INVAL, tx);
    }
    let Some(port) = super::driver_call::brightness_port(ctx) else {
        return respond::status(sender, req, E_NOENT, tx);
    };
    let (p, [lo, hi]) = (panel.to_le_bytes(), (value as u16).to_le_bytes());
    let msg = [OP_SET_VCP, p[0], p[1], p[2], p[3], code as u8, lo, hi];
    let status = match super::driver_call::status(port, &msg) {
        Some(0) => 0,
        Some(-1) => E_PERM,
        Some(-22) => E_INVAL,
        Some(-2) | Some(-38) => E_NOENT,
        _ => E_IO,
    };
    respond::status(sender, req, status, tx)
}
