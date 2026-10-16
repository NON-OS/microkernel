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

//! NCMP 0x0011 OUTPUT_ARRANGEMENT: the displays as they are laid out now
//! (`display::arrange_wire`), so Settings shows what the screens do rather
//! than what it last sent. Empty body. Only a trusted source asks (E_PERM
//! otherwise): the shell, for Settings, as with OUTPUT_LAYOUT and SCALE.

use crate::display::arrange_wire::{encode, Arrangement};
use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::server::trusted_pids::trusted_pids;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    if sender == 0 || !trusted_pids().contains(&Some(sender)) {
        return respond::status(sender, req, E_PERM, tx);
    }
    if !body.is_empty() {
        return respond::status(sender, req, E_INVAL, tx);
    }
    let a = Arrangement {
        outputs: 1 + ctx.heads.len() as u32,
        extend: ctx.want_extend,
        extended: ctx.plan.is_some(),
        scale_q: ctx.scale_q,
        chosen: ctx.output_scale,
    };
    respond::status_payload(sender, req, 0, &encode(&a), tx)
}
