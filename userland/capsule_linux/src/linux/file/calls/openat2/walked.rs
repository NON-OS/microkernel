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

/* The walk open makes, refused at the first step the rules forbid. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::walk::Step;
use super::super::super::{mounts, walk};
use super::open::{BENEATH, NO_MAGICLINKS, NO_SYMLINKS, NO_XDEV};

/*
 * Walk the name as open will, and refuse the first step `rules` forbid:
 * a link at all, a /proc magic link, an absolute link or a step out of the
 * starting directory under BENEATH, a step onto another mount.
 */
pub(super) fn walked(guest: &Guest, base: &[u8], named: &[u8], rules: u64) -> Result<(), i64> {
    let mount = mounts::of(base).0;
    let under = |p: &[u8]| {
        base == b"/" || p.starts_with(base) && matches!(p.get(base.len()), None | Some(b'/'))
    };
    /* The walk passes base's own parents on its way down to it. */
    let mut reached = false;
    let step = |s: Step| {
        match s {
            Step::Link { at, to } => {
                let magic = at.starts_with(b"/proc/")
                    && !at.ends_with(b"/self")
                    && !at.ends_with(b"/thread-self")
                    && !at.ends_with(b"/mounts");
                if rules & NO_SYMLINKS != 0 || (rules & NO_MAGICLINKS != 0 && magic) {
                    return Err(errno::ELOOP);
                }
                /* BENEATH allows neither an absolute link nor a magic one. */
                if rules & BENEATH != 0 && (to.first() == Some(&b'/') || magic) {
                    return Err(errno::EXDEV);
                }
            }
            Step::At(p) => {
                reached |= under(p);
                if reached && rules & BENEATH != 0 && !under(p) {
                    return Err(errno::EXDEV);
                }
                if reached && rules & NO_XDEV != 0 && mounts::of(p).0 != mount {
                    return Err(errno::EXDEV);
                }
            }
        }
        Ok(())
    };
    walk::walk(guest, named.to_vec(), true, step).map(|_| ())
}
