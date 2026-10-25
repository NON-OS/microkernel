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

//! The per-claim line the broker's confinement says: what a step did, with
//! no pid, requester id or stream id (K10), since the log leaves the machine
//! in photos and bug reports.

pub(super) fn say(what: &[u8]) {
    crate::sys::serial::print(b"[VT-D] ");
    crate::sys::serial::println(what);
}
