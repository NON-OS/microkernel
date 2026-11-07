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
//! What became of each of the shell's two frames, the desk and the chrome,
//! once a paint showed them, and so which of the parts it changed the caller
//! still commits to the compositor.
//!
//! A frame flipped in is damaged by the request that took it: a present by
//! the part it names, a whole submit by the whole surface, which is the
//! whole screen. A damage commit of the same part after it asked for the
//! same pixels again, and as the two requests come apart the compositor
//! composed them twice: the menu bar's strip each minute, a lit menu row,
//! the dock, and the whole screen after every whole repaint. Only what no
//! request the compositor took damaged is owed a commit. Pure, so the host
//! proofs hold it (desktop_proofs chrome_sent_tests).

/// One frame after it was shown.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sent {
    /// The same as the frame on screen: nothing was shown.
    Same,
    /// Presented with the first part it changed named.
    Presented,
    /// Submitted whole in place of the shown one.
    Submitted,
    /// Changed, with no request the compositor took to say so: drawn in
    /// place on the one backing, or its present and its submit refused.
    Unsaid,
}

impl Sent {
    /// The compositor took the frame: its backing is the shown one now.
    pub fn taken(self) -> bool {
        matches!(self, Sent::Presented | Sent::Submitted)
    }
}

/// The frame that is `new`, on a surface that `flips` between two backings,
/// shown by `flip`. `flip` is asked only for a new frame on a surface that
/// flips, and answers Presented, Submitted or Unsaid.
pub fn sent(new: bool, flips: bool, flip: impl FnOnce() -> Sent) -> Sent {
    match (new, flips) {
        (false, _) => Sent::Same,
        (true, false) => Sent::Unsaid,
        (true, true) => flip(),
    }
}

/// The parts of a change a caller still commits after its frame was shown.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Owed {
    /// None: the frame was on screen already, or a whole submit damaged all.
    Nothing,
    /// All but the first, which the present named.
    AfterFirst,
    /// Every one: a changed frame was not taken.
    Every,
}

impl Owed {
    /// The parts of `parts`, the first the one the frame was presented with,
    /// that are owed a commit.
    pub fn of<T>(self, parts: &[T]) -> &[T] {
        match self {
            Owed::Nothing => &[],
            Owed::AfterFirst => parts.get(1..).unwrap_or(&[]),
            Owed::Every => parts,
        }
    }
}

/// What a paint did with the desk's frame and the chrome's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Painted {
    pub desk: Sent,
    pub chrome: Sent,
}

impl Painted {
    /// A frame differs from the one that was on screen.
    pub fn changed(self) -> bool {
        self.desk != Sent::Same || self.chrome != Sent::Same
    }

    /// What the caller commits of the parts it changed. Both surfaces are
    /// the whole screen wide, so a submit of either damaged every part.
    pub fn owed(self) -> Owed {
        let both = [self.desk, self.chrome];
        if both.contains(&Sent::Unsaid) {
            Owed::Every
        } else if !self.changed() || both.contains(&Sent::Submitted) {
            Owed::Nothing
        } else {
            Owed::AfterFirst
        }
    }
}
