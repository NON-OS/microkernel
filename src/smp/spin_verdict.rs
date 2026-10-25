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

//! Whether a lock a wait has gone on too long for is held by the waiter's own
//! thread, pure: kernel_proofs::spin_waits.

use super::spin_watch::CHAIN_SPAN;

/// Who holds a lock a wait has gone on too long for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Holder {
    /// Somebody else, on another stack: the wait may yet end.
    Other,
    /// The holder's frame is near the waiter's, but either the waiter's own
    /// stack bounds are not known, or the take was a shared read that may
    /// already have been let go by its reader while another reader holds on.
    Likely,
    /// The holder's frame is on the waiter's own kernel stack. Only the
    /// waiter's own thread runs on that stack, so its own caller holds the
    /// lock and the wait can never end. Live stacks never overlap, so a hold
    /// on any other CPU is never this.
    ReTaken,
}

/// Judge the holder's frame at `holder_sp` against the waiter's at
/// `waiter_sp`, whose kernel stack ends at `stack_top` (0 when unknown). A
/// `shared` take is a reader's. Inlining can put both frames in one, in
/// either order, so only which stack they are on is asked, never which is
/// higher.
pub(crate) const fn judge(
    holder_sp: usize,
    waiter_sp: usize,
    stack_top: usize,
    shared: bool,
) -> Holder {
    let base = stack_top.saturating_sub(CHAIN_SPAN);
    let near = holder_sp.abs_diff(waiter_sp) < CHAIN_SPAN;
    if !shared && waiter_sp < stack_top && waiter_sp >= base {
        return if holder_sp < stack_top && holder_sp >= base {
            Holder::ReTaken
        } else {
            Holder::Other
        };
    }
    if near {
        Holder::Likely
    } else {
        Holder::Other
    }
}
