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

//! The locked session. While the shell's lock screen is up every event goes
//! to the shell, over focus, grabs and the trusted hold alike, and no
//! window sees a key go down. The one thing that still reaches a window is
//! the release of a key it saw go down before the lock, so it is not left
//! held. Only the shell sets or clears the lock, and it lasts until the
//! shell clears it: a shell that ends while locked leaves the router locked,
//! and its successor asks and finds it so. The lock word carries the
//! rectangles where the pointer is drawn (the lock's own fields); anywhere
//! else on a locked screen it is hidden.

use nonos_libc::InputEvent;

use crate::state::Context;

use super::held::deliver_held;
use super::key_deliver::deliver_key;
use super::kinds::is_keyboard;
use super::press_kinds::{is_pad_button, is_release, slot};

/// The word a lock request carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Word {
    Unlock,
    Lock,
    /// Change nothing; answer the state.
    Ask,
}

pub fn word(v: u32) -> Option<Word> {
    match v {
        0 => Some(Word::Unlock),
        1 => Some(Word::Lock),
        2 => Some(Word::Ask),
        _ => None,
    }
}

/// The most rectangles a lock word carries: one field on each output.
pub const MAX_SHOWN: usize = 4;

pub type Shown = [Option<(u32, u32, u32, u32)>; MAX_SHOWN];

/// A lock word's body: the word, then up to `MAX_SHOWN` rectangles
/// (x, y, width, height as u32 each) where the pointer is drawn while
/// locked. None when the body is not one.
pub fn parse(body: &[u8]) -> Option<(Word, Shown)> {
    let w = word(u32::from_le_bytes(body.get(0..4)?.try_into().ok()?))?;
    let rest = &body[4..];
    if !rest.len().is_multiple_of(16) || rest.len() / 16 > MAX_SHOWN {
        return None;
    }
    let mut shown: Shown = [None; MAX_SHOWN];
    for (slot, r) in shown.iter_mut().zip(rest.chunks_exact(16)) {
        let at = |i: usize| u32::from_le_bytes([r[i], r[i + 1], r[i + 2], r[i + 3]]);
        *slot = Some((at(0), at(4), at(8), at(12)));
    }
    Some((w, shown))
}

/// Whether the pointer is drawn at (x, y): always while unlocked; while
/// locked only inside one of the lock's rectangles.
pub fn pointer_shown(locked: bool, shown: &Shown, x: u32, y: u32) -> bool {
    !locked
        || shown
            .iter()
            .flatten()
            .any(|&(rx, ry, w, h)| x >= rx && y >= ry && x - rx < w && y - ry < h)
}

/// Only the shell may lock or unlock, or ask.
pub fn accept(sender: u32, shell: Option<u32>) -> bool {
    sender != 0 && shell == Some(sender)
}

/// The state after `w`.
pub fn after(locked: bool, w: Word) -> bool {
    match w {
        Word::Unlock => false,
        Word::Lock => true,
        Word::Ask => locked,
    }
}

const NS_PER_MS: u64 = 1_000_000;

/// The millisecond the session was last unlocked: moved only when an unlock
/// ends a lock, so a retold unlock or an ask leaves it where it was.
pub fn unlocked_at(locked: bool, w: Word, now_ms: i64, was: Option<u64>) -> Option<u64> {
    match (locked, w) {
        (true, Word::Unlock) => u64::try_from(now_ms).ok().or(was),
        _ => was,
    }
}

/// An event dated at or before the last unlock was posted while the lock
/// screen was up, and goes where the lock sends input however late it is
/// drained: the router serves the shell's unlock before it drains the ring,
/// so keys typed while the passphrase was checked would otherwise reach the
/// focused window. The kernel dates every posted event (input_ring
/// posted_at.rs) by the millisecond, so an event of the unlock's own
/// millisecond is taken as locked. An undated event is taken as posted now.
pub fn posted_locked(unlocked_ms: Option<u64>, posted_ns: u64) -> bool {
    unlocked_ms.is_some_and(|ms| posted_ns != 0 && posted_ns / NS_PER_MS <= ms)
}

/// Where a key or pad button goes while locked: a release to the process
/// that saw it go down, everything else to the shell.
pub fn key_target(release: bool, pressed_in: Option<u32>, shell: u32) -> u32 {
    match pressed_in {
        Some(p) if release && p != 0 => p,
        _ => shell,
    }
}

/// Route `event` while locked; the deliveries made.
pub fn route_locked(ctx: &mut Context, event: &InputEvent) -> u32 {
    let shell = super::pointer::shell_pid(ctx);
    if is_keyboard(event.kind) || is_pad_button(event.kind) {
        let release = is_release(event.kind);
        let pressed_in = ctx.key_targets.held_by(slot(event));
        return deliver_key(ctx, key_target(release, pressed_in, shell), event);
    }
    if shell == 0 {
        ctx.record(0);
        return 0;
    }
    let (n, pressed) = deliver_held(ctx, shell, event);
    ctx.record(n + pressed);
    n + pressed
}
