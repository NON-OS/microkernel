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

//! A path written as text, as a ResourceSource names its controller
//! ("\\_SB.PCI0.I2C1", "^GPO0", "I2C1"), found in the namespace. A single
//! name with no prefix is searched for from the scope upward to the root
//! (ACPI 6.5, 5.3); one with a prefix or several segments is not.

use crate::namespace::Namespace;
use crate::namespace::{Id, Seg, ROOT};

/// "I2C" becomes "I2C_": a NameSeg shorter than four is padded with '_'.
fn seg(text: &[u8]) -> Option<Seg> {
    let ok = !text.is_empty() && text.len() <= 4;
    let mut s = [b'_'; 4];
    s[..text.len().min(4)].copy_from_slice(&text[..text.len().min(4)]);
    ok.then_some(s)
}

pub fn resolve_text(ns: &Namespace, scope: Id, text: &[u8]) -> Option<Id> {
    let text = text.split(|b| *b == 0).next()?;
    let (mut at, mut rest, mut prefixed) = (scope, text, false);
    if let Some(r) = rest.strip_prefix(b"\\") {
        (at, rest, prefixed) = (ROOT, r, true);
    }
    while let Some(r) = rest.strip_prefix(b"^") {
        (at, rest, prefixed) = (ns.get(at)?.parent, r, true);
    }
    let segs: Option<alloc::vec::Vec<Seg>> = rest.split(|b| *b == b'.').map(seg).collect();
    let segs = segs?;
    if let ([one], false) = (segs.as_slice(), prefixed) {
        let mut s = scope;
        for _ in 0..256 {
            if let Some(found) = ns.child(s, *one) {
                return Some(found);
            }
            if s == ROOT {
                return None;
            }
            s = ns.get(s)?.parent;
        }
        return None;
    }
    segs.iter().try_fold(at, |id, s| ns.child(id, *s))
}
