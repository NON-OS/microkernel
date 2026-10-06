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

/* Whether a new name may be made where it is asked for. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::meta::stat;
use super::super::resolve::key;

/*
 * A new name must not exist as a file or a link, and must be somewhere the
 * guest may write: the shared tree is read-only to it.
 */
pub(super) fn free_and_writable(guest: &Guest, at: &[u8]) -> Result<(), i64> {
    if stat::look(at).is_some() || guest.links.target(at).is_some() {
        return Err(errno::EEXIST);
    }
    key(at).writable().map_err(|_| errno::EROFS)
}
