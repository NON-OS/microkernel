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

//! LOCK_PROMPT (0x0011) and LOCK_STANDING (0x0012), from Settings only
//! (`E_PERM` otherwise), empty bodies. The prompt opens the shell's own
//! passphrase prompt in the trusted band, so Settings never sees the
//! passphrase; the standing answers u32 1 when one is set, 0 when not.

use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::handlers::instances::each_live_pid;
use crate::server::respond;
use crate::state::Context;

const SETTINGS: &[u8] = b"app.settings";

fn from_settings(sender: u32) -> bool {
    sender != 0 && each_live_pid(SETTINGS, |pid| pid == sender)
}

pub fn prompt(ctx: &mut Context, sender: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !from_settings(sender) {
        let _ = respond::status(sender, req, E_PERM, tx);
        return;
    }
    if !body.is_empty() {
        let _ = respond::status(sender, req, E_INVAL, tx);
        return;
    }
    // Answered first: the prompt holds the shell until it is done, and
    // Settings must not wait on the person.
    let _ = respond::status(sender, req, 0, tx);
    crate::server::lock::ask_change(ctx);
}

pub fn standing(ctx: &Context, sender: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !from_settings(sender) {
        let _ = respond::status(sender, req, E_PERM, tx);
        return;
    }
    if !body.is_empty() {
        let _ = respond::status(sender, req, E_INVAL, tx);
        return;
    }
    let set = u32::from(ctx.lock.offered());
    let _ = respond::payload(sender, req, &set.to_le_bytes(), tx);
}
