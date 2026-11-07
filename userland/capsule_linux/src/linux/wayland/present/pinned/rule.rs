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

//! When a commit is shown from its pinned buffer, and when a buffer the
//! window showed so is the client's again. Pure, so capsule_linux_proofs
//! holds it.

/// What the window and the commit are, as show.rs reads them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Facts {
    /// The window shows this surface, on buffers of its own.
    pub on: bool,
    /// Subsurfaces are laid on it, or their pixels kept: the bridge
    /// composes those into its own buffers.
    pub layered: bool,
    /// A draw is owed, or a configure's fit is due: drawn whole by the copy.
    pub owes: bool,
    /// The buffer is of the window's shape, at its origin, and its frame
    /// is the one the window's buffers are laid at.
    pub same_shape: bool,
    /// The frame's look is the one drawn: no control to light first.
    pub chrome_current: bool,
    /// Shown at its own size, not scaled: all of it, or the box of it its
    /// window geometry names, which the compositor draws as that box.
    pub plain: bool,
    /// Its rows lie in its pool and in one mapping of its memfd, from a
    /// page boundary in the guest.
    pub pinnable: bool,
    /// The window shows its own buffers and its client committed to it
    /// before: this commit is not one of a run of frames, or comes
    /// GIVE_BACK_MS or more after the last while its client has a spare
    /// (paced).
    pub paced: bool,
}

/// Whether the commit is shown from its pinned buffer.
pub fn pinned(f: &Facts) -> bool {
    f.on && !f.layered
        && !f.owes
        && f.same_shape
        && f.chrome_current
        && f.plain
        && f.pinnable
        && !f.paced
}

/// A window that showed one buffer this long with no commit gives it back
/// drawn into its own: a client with one buffer waits for its release
/// before it draws again, and would wait for good.
pub const IDLE_MS: u64 = 250;

/// Whether a content shown since `since` has waited out IDLE_MS by `now`.
pub fn idle(since: u64, now: u64) -> bool {
    now.saturating_sub(since) >= IDLE_MS
}

/// Whether a window shown from one buffer since `since` gives it back by
/// `now`: once idle while its client has no `spare`, another buffer of that
/// shape that is its own again, and after GIVE_BACK_MS while it has one. A
/// client drawing in two in turn, as foot does, draws its next frame in the
/// spare; taking its shown one back would draw the window whole twice, into
/// its own buffers and over the ground again when the next is pinned. The
/// longer bound still gives the buffer back to a client that waits for that
/// one alone.
pub fn gives_back(since: u64, now: u64, spare: bool) -> bool {
    if spare {
        now.saturating_sub(since) >= GIVE_BACK_MS
    } else {
        idle(since, now)
    }
}

/// A commit sooner than this after the one before it is one of a run of
/// frames: 10 a second or more. A person typing or selecting commits
/// slower, at 200 to 300 ms a key, close to IDLE_MS on either side.
pub const RUN_MS: u64 = 100;

/// Commits in a row, each sooner than RUN_MS after the one before, that are
/// a run of frames: a scroll, a drag or an animation. foot draws a key or a
/// pointer step in two commits 30 to 80 ms apart; that pair alone is no run.
pub const RUN_FRAMES: u32 = 3;

/// The commits in a row, `was` before this one at `now`, each sooner than
/// RUN_MS after its client's commit at `last` before it: none when `last`
/// is unknown or as long ago.
pub fn run(was: u32, last: Option<u64>, now: u64) -> u32 {
    match last {
        Some(t) if now.saturating_sub(t) < RUN_MS => was.saturating_add(1),
        _ => 0,
    }
}

/// Whether a commit at `now` to a window drawn on its own buffers is drawn
/// by the copy, only the box said changed, and not pinned. `idle_back` is
/// its client's last commit since the window gave its pin back at idle,
/// `own_last` its last while the window showed its own buffers, `run` this
/// commit's place in a run (run above); a window with neither has shown no
/// commit of its client yet, and is not paced.
///
/// A client with no `spare` has one buffer to draw in, and a pin holds it:
/// unless it commits again within IDLE_MS, the window gives the buffer back
/// (gives_back). Pinned, a commit is drawn over a fresh ground, the ground
/// and the content composed whole, and the give-back reads the buffer out
/// of the guest whole and composes it whole again: four whole frames, where
/// the copy reads and composes only the box. Keys typed 200 ms apart, the
/// second commit foot makes 30 ms after the first for a key, and a slow
/// client's first commit after the window was drawn by the copy, came so
/// on a guest. Only a run of RUN_FRAMES is pinned: its client draws on,
/// finds its buffer held and draws its next frame in another.
///
/// A client with a `spare` draws its next frame in that one, so its pin is
/// kept to GIVE_BACK_MS: a commit that long or more after its last is drawn
/// by the copy, as the pin would have been given back by then, and a sooner
/// one is pinned.
pub fn paced(
    idle_back: Option<u64>,
    own_last: Option<u64>,
    run: u32,
    now: u64,
    spare: bool,
) -> bool {
    if spare {
        let waited = |t: Option<u64>| t.is_some_and(|t| now.saturating_sub(t) >= GIVE_BACK_MS);
        return waited(idle_back) || waited(own_last);
    }
    (idle_back.is_some() || own_last.is_some()) && run < RUN_FRAMES
}

/// How long a buffer taken off a layer waits for its release count: a
/// compositor that composes nothing (the lid shut, the screen blank) moves
/// none, and its client would wait for good. Given back then, the most a
/// client drawing early does is draw a frame the compositor shows half new.
pub const GIVE_BACK_MS: u64 = 1000;

/// Whether a buffer taken off the layer `waited` ms ago is the client's
/// again: the compositor moved its release count past `seen`, or the
/// kernel no longer has the surface (a negative answer), so nothing reads
/// it; or it waited GIVE_BACK_MS.
pub fn given_back(seen: u32, now: i64, waited: u64) -> bool {
    now < 0 || now != i64::from(seen) || waited >= GIVE_BACK_MS
}
