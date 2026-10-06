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

//! Which brought-up controller one capsule instance serves. Each is tried in
//! `rank::try_order`; what a namespace holds is known only once its
//! controller is up, so the choice is made from how each try went. Pure, so
//! the host proofs hold it.

/// How bringing one controller up went.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The bring-up failed; nothing of it is held.
    Failed,
    /// Up, but its namespace got no I/O queue (none, empty, or a format the
    /// driver does not serve): identify and health only.
    NoIo,
    /// Up with an I/O queue on a non-empty namespace.
    Io,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    /// Serve the controller at this index of the tries.
    Serve(usize),
    /// Fail this attempt so the bring-up schedule tries again.
    Retry,
}

/// Whether the tries can stop after this one: a disk (not a cache module)
/// with an I/O queue is the best there is.
pub const fn settled(cache: bool, outcome: Outcome) -> bool {
    !cache && matches!(outcome, Outcome::Io)
}

/// The choice from `tries`, each (is a cache module, outcome) in the order
/// tried. A disk with an I/O queue is served first. A disk that failed to
/// come up is retried before anything lesser is served: it may be the
/// internal SSD, slow after an unclean shutdown, and serving the cache
/// module or an empty namespace instead would hold the boot to the wrong
/// disk for good. With every disk up and none holding I/O, a cache module
/// with I/O is served, and then the first controller that came up at all,
/// so identify and health still answer.
pub fn choose(tries: &[(bool, Outcome)]) -> Choice {
    let first = |want: &dyn Fn(bool, Outcome) -> bool| {
        tries.iter().position(|&(cache, outcome)| want(cache, outcome))
    };
    if let Some(i) = first(&|cache, o| !cache && o == Outcome::Io) {
        return Choice::Serve(i);
    }
    if first(&|cache, o| !cache && o == Outcome::Failed).is_some() {
        return Choice::Retry;
    }
    if let Some(i) = first(&|_, o| o == Outcome::Io) {
        return Choice::Serve(i);
    }
    match first(&|_, o| o == Outcome::NoIo) {
        Some(i) => Choice::Serve(i),
        None => Choice::Retry,
    }
}
