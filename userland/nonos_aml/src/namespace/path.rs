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

//! An object's absolute path, for logs and for the parser's scope tracking.

use super::{Id, Namespace, Seg, ROOT};
use alloc::vec::Vec;
use core::fmt;

/// The NameSegs from the root down to `id`, the root itself excluded.
pub fn absolute(ns: &Namespace, mut id: Id) -> Vec<Seg> {
    let mut segs = Vec::new();
    while id != ROOT {
        let Some(e) = ns.get(id) else { break };
        segs.push(e.seg);
        id = e.parent;
    }
    segs.reverse();
    segs
}

/// Writes `id`'s path as ASL spells it: `\_SB_.PCI0.LPCB`.
pub fn write_path(ns: &Namespace, id: Id, out: &mut impl fmt::Write) -> fmt::Result {
    out.write_char('\\')?;
    for (i, seg) in absolute(ns, id).iter().enumerate() {
        if i > 0 {
            out.write_char('.')?;
        }
        for b in seg {
            out.write_char(*b as char)?;
        }
    }
    Ok(())
}
