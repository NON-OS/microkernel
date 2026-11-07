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

//! Whether a request's sender answers for the surface it names. A handle the
//! compositor can attach is any surface shared with it, and a handle is a
//! slot and a small counter, easy to guess: without this one client could
//! submit, present or draw as content another's window buffer, or a guest's
//! pinned buffer, and show it as its own window.

use crate::state::answer_rule::{answers, capture_target};
use crate::state::attach_kernel::Kernel;
use crate::state::Context;

/// Whether `sender` answers for the surface `handle`, as the kernel's
/// descriptor names who does: the surface's owner or, for one pinned from
/// a guest, the guest's supervisor (state/answer_rule.rs).
pub(super) fn answers_for(ctx: &mut Context, sender: u32, handle: u64) -> bool {
    let screen = screen_view(ctx, handle);
    answers(&mut Kernel, screen, ctx.attach.holds(handle), sender, handle)
}

/// Whether a screenshot may be written into `handle` for `sender`: its own
/// surface, of which the compositor holds no view (state/answer_rule.rs).
pub(super) fn capture_into(ctx: &mut Context, sender: u32, handle: u64) -> bool {
    let screen = screen_view(ctx, handle);
    capture_target(&mut Kernel, screen, ctx.attach.holds(handle), sender, handle)
}

/// The views of the screen held outside the cache: the primary or backing
/// drawn into, and each further output's primary.
fn screen_view(ctx: &Context, handle: u64) -> bool {
    handle == ctx.surface_handle || ctx.heads.iter().any(|h| h.handle == handle)
}
