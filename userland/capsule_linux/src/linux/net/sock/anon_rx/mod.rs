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

//! Reading a net.anon stream. net.anon has no way to ask whether bytes are
//! waiting without taking them, so poll and read both take what has arrived
//! into the socket's own buffer while it is empty, and a read drains it. The
//! buffer holds at most one reply's bytes (PAYLOAD_MAX): it is filled only
//! when empty, and a fill longer than that is refused as a broken answer.

mod bits;
mod fill;
mod got;
mod take;

pub use got::Got;
