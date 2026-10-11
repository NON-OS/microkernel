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

//! A net.anon stream's receive buffer. net.anon cannot be asked whether
//! bytes wait without taking them, so poll and read take them into the
//! socket's buffer and a read drains it. Held here: the buffer never holds
//! more than one reply, bytes are never put behind bytes or after the end,
//! poll never calls a stream readable on an empty buffer while it is open,
//! and a read reports the far end the way Linux reports a TCP peer.

mod after;
mod bound;
mod ends;
mod held;
