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

use super::consts::MAX_ALGS;
use super::error::LogError;
use super::read::Stop;

/// Where a walk over a prefix of some bytes stands.
#[derive(Debug, PartialEq, Eq)]
pub enum Walk<T> {
    /// The prefix holds all of it.
    Done(T),
    /// The prefix ends early; all of it needs at least this many bytes.
    Needs(usize),
}

impl<T> Walk<T> {
    pub fn done(self) -> Option<T> {
        match self {
            Walk::Done(t) => Some(t),
            Walk::Needs(_) => None,
        }
    }

    pub(crate) fn of(r: Result<T, Stop>) -> Result<Walk<T>, LogError> {
        match r {
            Ok(t) => Ok(Walk::Done(t)),
            Err(Stop::Short(n)) => Ok(Walk::Needs(n)),
            Err(Stop::Bad(e)) => Err(e),
        }
    }
}

/// How often `grow` widens its view: a header or an event names its lengths in
/// a few steps per bank.
const MAX_STEPS: usize = 4 * MAX_ALGS + 8;

/*
 * Walk bytes the caller can only view a prefix of at a time, the firmware's log
 * buffer before it is copied: view `first` bytes, and each time the walk needs
 * more, view exactly that much, never past `limit`. `view(n)` gives the first
 * `n` bytes or `None`. Nothing is read that the structure does not name.
 */
pub fn grow<'a, T>(
    first: usize,
    limit: usize,
    mut view: impl FnMut(usize) -> Option<&'a [u8]>,
    walk: impl Fn(&[u8]) -> Result<Walk<T>, LogError>,
) -> Result<T, LogError> {
    let mut n = first;
    for _ in 0..MAX_STEPS {
        if n > limit {
            return Err(LogError::TooLarge);
        }
        match walk(view(n).ok_or(LogError::Truncated)?)? {
            Walk::Done(t) => return Ok(t),
            Walk::Needs(m) if m > n => n = m,
            Walk::Needs(_) => return Err(LogError::Truncated),
        }
    }
    Err(LogError::Truncated)
}
