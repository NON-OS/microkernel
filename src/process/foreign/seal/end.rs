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

//! Where a family ends: when its last guest leaves the registry, by exit or
//! by teardown. The next MkForeignSpawn opens a new one.

use super::super::registry::guest_count;
use super::finish::{drop_family, finish};

/*
 * `pid` is leaving, hosted by `host` if it was a guest; `whole` says no
 * guest of its own outlived it. As a guest it may have been its host's last.
 * As a supervisor its family ends now: final if `whole`, and otherwise
 * dropped, since a guest still on another CPU is out of the registry and its
 * calls would go uncounted.
 */
pub(crate) fn ended(pid: u32, host: Option<u32>, whole: bool) {
    if let Some(sup) = host.filter(|&s| guest_count(s) == 0) {
        finish(sup);
    }
    if whole {
        finish(pid);
    } else {
        drop_family(pid);
    }
}
