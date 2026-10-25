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

//! The live families and the sealed ones, both bounded.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use spin::Mutex;

use super::family::Family;
use super::sealed::Sealed;

/// The oldest sealed record goes first once this many are kept.
pub(crate) const MAX_SEALED: usize = 64;

pub(crate) static LIVE: Mutex<Vec<Family>> = Mutex::new(Vec::new());
pub(crate) static SEALED: Mutex<VecDeque<Sealed>> = Mutex::new(VecDeque::new());

/// `sup` supervises a live family: a DELIVER may be queued for it.
pub(crate) fn supervises_live(sup: u32) -> bool {
    LIVE.lock().iter().any(|fam| fam.sup == sup)
}

/// `f` over the live family of `sup`, if it has one.
pub(crate) fn with_family<R>(sup: u32, f: impl FnOnce(&mut Family) -> R) -> Option<R> {
    LIVE.lock().iter_mut().find(|fam| fam.sup == sup).map(f)
}
