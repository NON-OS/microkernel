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

//! A tick that interrupted a guest thread, handed to the foreign layer.

use crate::process::userspace::types::UserContext;

/// A guest thread its supervisor asked to stop is parked here, running no
/// code, until it is answered. It resumes from the trampoline's frame `ctx`,
/// which a signal answer rewrites to enter the handler, with the FPU loaded
/// from `fx`, which the same answer rewrites to the handler's clean state.
pub(super) fn on_user_tick(ctx: *mut UserContext, fx: *mut u8) {
    let words = ctx.cast::<[u64; crate::process::foreign::TICK_FRAME_WORDS]>();
    /*
     * SAFETY: eK@nonos.systems - the trampoline's 160-byte frame, still on
     * this thread's kernel stack and restored from on the way out; the 20
     * words are exactly that frame, nothing past it.
     */
    crate::process::foreign::on_user_tick(unsafe { &mut *words }, fx);
}
