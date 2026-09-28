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

/* RESOLVE_IN_ROOT's walk and the magic links it never follows. */

use crate::linux::abi::errno;

use super::super::super::mounts;
use super::super::super::walk::Step;
use super::open::{NO_SYMLINKS, NO_XDEV};

/*
 * A step of an IN_ROOT walk: the walk keeps itself under the root, so
 * only the link and mount rules can refuse a step. A magic link would
 * reach past the root, and IN_ROOT never follows one.
 */
pub(super) fn rooted(step: Step, rules: u64, mount: u32) -> Result<(), i64> {
    match step {
        Step::Link { at, .. } if rules & NO_SYMLINKS != 0 || magic(at) => Err(errno::ELOOP),
        Step::At(p) if rules & NO_XDEV != 0 && mounts::of(p).0 != mount => Err(errno::EXDEV),
        _ => Ok(()),
    }
}

/*
 * A /proc link that names an object rather than a path: fd/N, exe, cwd,
 * root. /proc/self, thread-self and mounts are ordinary links.
 */
pub(super) fn magic(at: &[u8]) -> bool {
    at.starts_with(b"/proc/")
        && !at.ends_with(b"/self")
        && !at.ends_with(b"/thread-self")
        && !at.ends_with(b"/mounts")
}
