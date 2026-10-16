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

//! A device's reserved regions leaving its capsule's domain (L60).

extern crate alloc;

use super::reserved_rule::Held;
use super::table::Capsule;

/// Forget `device_id`'s regions, unmapping each no other device holds.
pub(super) fn unmap_reserved(c: &mut Capsule, device_id: u64) {
    let mine: alloc::vec::Vec<Held> =
        c.reserved.iter().copied().filter(|h| h.device == device_id).collect();
    c.reserved.retain(|h| h.device != device_id);
    for h in mine {
        if !c.reserved.iter().any(|o| o.base == h.base && o.end == h.end) {
            let _ = c.domain.unmap(h.base, (h.end - h.base) as usize);
        }
    }
}
