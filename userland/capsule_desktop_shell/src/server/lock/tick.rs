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

//! Each tick: write a blob not yet on disk, hand the blob back to a keyring
//! that lost it, tell the router again while locked, and ask once a boot
//! for a passphrase where none is set. The locked clock is shown by
//! refresh_clock, the band it changed alone (lock::geometry::tick_bounds).

use super::{keep, router, start};
use crate::state::Context;

const STANDING_MS: i64 = 5000;

pub fn tick(ctx: &mut Context, now_ms: i64) {
    if let Some(blob) = ctx.lock.verifier.due_write().map(alloc::vec::Vec::from) {
        if keep::write(&blob) {
            ctx.lock.verifier.written();
        }
    }
    if now_ms.saturating_sub(ctx.lock.standing_ms) >= STANDING_MS {
        ctx.lock.standing_ms = now_ms;
        let answered = start::refresh(ctx);
        let lost = ctx.lock.verifier.known() == crate::state::lock::Known::Absent;
        if answered && lost && !ctx.lock.verifier.blob().is_empty() {
            let blob = ctx.lock.verifier.blob().to_vec();
            start::load(ctx, &blob);
        }
    }
    if ctx.lock.screen.covers() {
        if now_ms.saturating_sub(ctx.lock.told_ms) >= router::RETELL_MS {
            router::tell(ctx);
        }
        return;
    }
    if ctx.lock.verifier.ask_due() && ctx.lock.choose.is_none() {
        super::ask_first(ctx);
    }
}
