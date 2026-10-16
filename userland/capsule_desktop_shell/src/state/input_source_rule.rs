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

//! Input events are believed from the input router alone. Anything can send
//! the shell a frame shaped like a delivery; a window that did could type
//! into a prompt, or forge the secure attention chord. The kernel stamps the
//! sender's pid on every message, and only the router's is taken.

/// Whether a delivery from `sender` is the router's, given the router's pid
/// as the service registry names it (None while it is not running).
pub fn from_router(sender: u32, router: Option<u32>) -> bool {
    sender != 0 && router == Some(sender)
}
