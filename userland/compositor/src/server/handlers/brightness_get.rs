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

//! NCMP 0x0016 BRIGHTNESS_GET: each panel's brightness now, from the shell
//! alone, which passes it to Settings to show; nothing here changes a
//! level. Every answer carries the fixed payload, zeros beside a refusal.

use crate::display::brightness_get::{encode, BRIGHTNESS_GET_LEN};
use crate::display::brightness_list::{panels, MAX_PANELS};
use crate::display::brightness_rule::OP_BRIGHTNESS;
use crate::protocol::{Request, E_INVAL, E_IO, E_NOENT, E_PERM};
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let refuse = |status, tx: &mut [u8]| {
        respond::status_payload(sender, req, status, &[0; BRIGHTNESS_GET_LEN], tx)
    };
    if !super::shell_only::from_shell(sender) {
        return refuse(E_PERM, tx);
    }
    if !body.is_empty() {
        return refuse(E_INVAL, tx);
    }
    let Some(port) = super::driver_call::brightness_port(ctx) else { return refuse(E_NOENT, tx) };
    let mut reply = [0u8; 512];
    let Some(n) = super::driver_call::call(port, &[OP_BRIGHTNESS], &mut reply) else {
        return refuse(E_IO, tx);
    };
    let mut list = [None; MAX_PANELS];
    let k = panels(&reply[..n], &mut list);
    let found: alloc::vec::Vec<_> = list.iter().take(k).flatten().copied().collect();
    respond::status_payload(sender, req, 0, &encode(&found), tx)
}
