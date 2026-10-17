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

use super::table::Capsule;

/// Take back the last `added` ranges `reserve` put in, when the device they
/// were for could not be attached after all.
pub(super) fn unreserve(c: &mut Capsule, added: usize) {
    for range in c.reserved.split_off(c.reserved.len().saturating_sub(added)) {
        let _ = c.domain.unmap(range.iova, range.len as usize);
    }
}
