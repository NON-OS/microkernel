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

//! At start: what the keyring holds, the blob on disk handed back to it
//! when it holds nothing, and the router's own lock, which a shell that
//! restarted while locked finds set and covers again.

use nonos_lock::wire::{answer, parse_standing, Answer};

use super::{cover, keep, router};
use crate::state::lock::Known;
use crate::state::Context;

pub fn start(ctx: &mut Context) {
    refresh(ctx);
    if ctx.lock.verifier.known() != Known::Set {
        if let Some(blob) = keep::read() {
            load(ctx, &blob);
        }
    }
    if router::ask(ctx) == Some(true) {
        // The screen was locked when this shell started: it stays locked
        // whatever the keyring holds, until the right passphrase.
        cover(ctx);
    }
}

/// Ask the keyring what it holds. False when it did not answer.
pub(super) fn refresh(ctx: &mut Context) -> bool {
    let rid = ctx.issue_request_id();
    let Some((status, body)) = crate::keyring_client::state(rid) else {
        return false;
    };
    let Answer::Done(body) = answer(status, &body) else {
        return false;
    };
    let Some((set, count, blob)) = parse_standing(body) else {
        return false;
    };
    ctx.lock.verifier.standing(set, blob);
    let now = nonos_libc::mk_uptime_ms();
    ctx.lock.screen.hold_for(count.wait_ms, now);
    true
}

/// Hand `blob` to a keyring that holds nothing. Taken: the shell's copy is
/// the one on disk. Refused by this machine (a sealed record after the boot
/// state changed, or a clear one where there is a TPM): none is set, and
/// the person is asked for a new one.
pub(super) fn load(ctx: &mut Context, blob: &[u8]) {
    let rid = ctx.issue_request_id();
    let Some((status, body)) = crate::keyring_client::load(rid, blob) else {
        return;
    };
    match answer(status, &body) {
        Answer::Done(_) => {
            ctx.lock.verifier.loaded(blob);
            refresh(ctx);
        }
        Answer::Held => {
            refresh(ctx);
        }
        _ => {}
    }
}
