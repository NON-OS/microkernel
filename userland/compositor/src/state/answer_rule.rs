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

//! Whether a request's sender answers for the surface it names, pure over
//! the kernel's two calls, so compositor_proofs holds it with a kernel
//! double (answer_rule_tests).
//!
//! The kernel names who answers for a surface only in an attach's
//! descriptor. A surface the cache maps already is asked from that view,
//! which the kernel answers from its record with no new mapping. Any other
//! is asked through a view of its own, let go again whatever the answer:
//! no cache slot is taken, so a handle a client guesses evicts no mapping a
//! window is drawn from and leaves no map of another's surface, and a full
//! cache refuses no client its own new buffer. The composite maps the
//! surface into the cache when it draws it. A view the compositor keeps of
//! the screen itself, outside the cache, is never asked: that release would
//! unmap what the compositor draws into.

use super::content_rule::may_show;

/// The kernel calls the question makes.
pub trait Owners {
    /// The process that answers for `handle`, from the descriptor of a view
    /// of it, kept; None when the kernel will not let this process view it.
    fn owner(&mut self, handle: u64) -> Option<u32>;
    /// Drop this process's view of `handle`.
    fn release(&mut self, handle: u64);
    /// The process `sender`'s thread belongs to, as the kernel names it.
    /// A surface's owner is a process, and IPC names the sending thread.
    fn process_of(&mut self, sender: u32) -> u32;
}

/// Whether `sender` answers for `handle`. `screen`: it is a view of the
/// screen the compositor holds outside the cache (its primary or backing,
/// a further output's primary). `cached`: the attach cache maps it now.
pub fn answers(k: &mut impl Owners, screen: bool, cached: bool, sender: u32, handle: u64) -> bool {
    if handle == 0 || screen {
        return false;
    }
    let owner = k.owner(handle);
    if !cached && owner.is_some() {
        k.release(handle);
    }
    may_show(owner, k.process_of(sender))
}

/// Whether a screenshot may be written into `handle` for `sender`: a
/// surface the sender answers for that the compositor holds no view of. The
/// capture attaches the surface and lets it go when done. For one already
/// viewed, the screen's or one the cache draws a layer from, that attach
/// hands back the compositor's own view, the capture is written over its
/// pixels, and the release unmaps it under the compositor.
pub fn capture_target(
    k: &mut impl Owners,
    screen: bool,
    cached: bool,
    sender: u32,
    handle: u64,
) -> bool {
    !cached && answers(k, screen, false, sender, handle)
}
