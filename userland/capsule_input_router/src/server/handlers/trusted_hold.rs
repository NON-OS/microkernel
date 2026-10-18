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

//! NIRS op 0x0005, from the compositor: the trusted band's holder, a pid
//! u32 (0: none). It is one-way and never answered: the compositor sends it
//! without waiting, and an answer it is not waiting for would come back to
//! it as a frame it cannot read. A notice from any other process, or of any
//! other length, is dropped.

use nonos_libc::mk_uptime_ms;

use crate::clients::wire::lookup_pid;
use crate::protocol::read_u32;
use crate::route::trusted_hold::{accept, told};
use crate::state::Context;

const COMPOSITOR: &[u8] = b"compositor";

pub fn handle(ctx: &mut Context, sender_pid: u32, body: &[u8]) {
    if body.len() != 4 || !accept(sender_pid, lookup_pid(COMPOSITOR)) {
        return;
    }
    if let Some(pid) = read_u32(body, 0) {
        ctx.trusted = told(pid, mk_uptime_ms());
    }
}
