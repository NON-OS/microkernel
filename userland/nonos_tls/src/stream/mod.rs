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

//! A TLS 1.3 session that stays open across many reads and writes.

mod absorb;
mod connect;
mod content;
mod dispatch;
mod finished;
mod gather;
mod handshake_keys;
mod io;
mod limits;
mod open_record;
mod seal;
mod settle;
mod span;
mod span_scan;
mod types;

pub use connect::connect_unauthenticated;
pub use types::Stream;
