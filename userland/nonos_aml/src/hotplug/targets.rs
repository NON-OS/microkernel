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

//! The devices a check covers, and whether each is there now.

use crate::load::Aml;
use crate::namespace::{Id, Kind};
use crate::platform::{sta, Eval, PRESENT};
use alloc::vec::Vec;

/// More than any slot, bridge or dock holds below it.
const MAX_TARGETS: usize = 256;

/// The notified device, then (when `deep`) every device below it, each
/// parent before its children, in definition order.
pub fn targets(aml: &Aml, id: Id, deep: bool) -> Vec<Id> {
    let mut out = alloc::vec![id];
    let mut at = 0;
    while deep && at < out.len() && out.len() < MAX_TARGETS {
        let kids = aml.ns.children(out[at]);
        let devices = kids.filter(|k| aml.ns.get(*k).is_some_and(|e| e.kind == Kind::Device));
        let room = MAX_TARGETS - out.len();
        let found: Vec<Id> = devices.take(room).collect();
        out.extend(found);
        at += 1;
    }
    out
}

/// None when `_STA` faulted: a device firmware cannot answer for is left
/// as it was, neither probed nor taken away.
pub fn present(aml: &mut Aml, eval: &mut Eval<'_>, id: Id) -> Option<bool> {
    sta(aml, eval, id).ok().map(|s| s & PRESENT != 0)
}
