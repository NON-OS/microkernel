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

//! net.socks5's own reader of a caller's frames (capsule_socks5
//! server/request/), so the frames a named stream sends are held to the
//! parser that takes them, not to a copy of its markers.

#[path = "../../capsule_socks5/src/server/request/marks.rs"]
mod marks;

#[path = "../../capsule_socks5/src/server/request/ask.rs"]
pub mod ask;
