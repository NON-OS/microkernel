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

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::synth::{self, Node};
use super::super::super::{mounts, resolve, walk};
use super::open::{BENEATH, NO_MAGICLINKS, NO_SYMLINKS, NO_XDEV};

/*
 * Walk the name a component at a time from where it starts, and check each
 * step against `rules`.
 */
pub(super) fn walked(guest: &Guest, base: &[u8], named: &[u8], rules: u64) -> Result<(), i64> {
    let mount = mounts::of(base).0;
    let below = base == b"/"
        || named.starts_with(base) && matches!(named.get(base.len()), None | Some(b'/'));
    let (mut at, rest) = match below && base != b"/" {
        true => (base.to_vec(), &named[base.len()..]),
        false => (Vec::new(), named),
    };
    for part in rest.split(|b| *b == b'/').filter(|p| !p.is_empty()) {
        at.push(b'/');
        at.extend_from_slice(part);
        let link = guest.links.target(&at).is_some();
        let made = matches!(synth::node(&at), Some(Ok(Node::Link(_))));
        let magic = made
            && at.starts_with(b"/proc/")
            && !at.ends_with(b"/self")
            && !at.ends_with(b"/thread-self");
        if (rules & NO_SYMLINKS != 0 && (link || made)) || (rules & NO_MAGICLINKS != 0 && magic) {
            return Err(errno::ELOOP);
        }
        if link || made {
            at = walk::follow(guest, core::mem::take(&mut at), true);
        }
        if rules & NO_XDEV != 0 && mounts::of(&at).0 != mount {
            return Err(errno::EXDEV);
        }
    }
    let end = resolve::visible(b"/", &at);
    let inside =
        base == b"/" || end.starts_with(base) && matches!(end.get(base.len()), None | Some(b'/'));
    if rules & BENEATH != 0 && !inside {
        return Err(errno::EXDEV);
    }
    Ok(())
}
