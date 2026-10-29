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

//! The ended child a wait takes from its parent's lists: the first that
//! fits, out of both lists unless WNOWAIT. Pure, so the host proofs hold
//! which child a wait reaps.

use alloc::vec::Vec;

use super::wait_opts::{WEXITED, WNOWAIT};

/// What one wait finds among a parent's children.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Found {
    /// This ended child, with its wait status: reaped unless WNOWAIT.
    Ended(u32, i32),
    /// A child that fits has not ended yet.
    NotYet,
    /// No child fits at all, which is ECHILD.
    NoChild,
}

/// The first of `ended` that `fits`, when WEXITED asks for ended children,
/// else whether any of `children` fits.
pub fn take(
    ended: &mut Vec<(u32, i32)>,
    children: &mut Vec<u32>,
    options: u64,
    fits: impl Fn(u32) -> bool,
) -> Found {
    let exits = options & WEXITED != 0;
    let Some(at) = ended.iter().position(|(pid, _)| exits && fits(*pid)) else {
        return match children.iter().any(|c| fits(*c)) {
            true => Found::NotYet,
            false => Found::NoChild,
        };
    };
    let (pid, status) = ended[at];
    if options & WNOWAIT == 0 {
        ended.remove(at);
        children.retain(|c| *c != pid);
    }
    Found::Ended(pid, status)
}
