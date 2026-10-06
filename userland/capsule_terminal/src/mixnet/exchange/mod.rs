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

//! One HTTP request and response over the chosen network, stepped from a
//! terminal job instead of waited out on the window's thread. `curl` used to
//! hold the window for as long as a page took through the mixnet (minutes at
//! worst), with nothing drawn and Ctrl+C unread; `git clone` held it for the
//! whole transfer.
//!
//! The route is the network the person chose (`Route::chosen`), the rule
//! every capsule holding Network leaves by: the Nym mixnet through
//! net.socks5, the Anyone network through net.anon, a direct socket only
//! when Direct is the default, and when that network is not running, no
//! connection and the reason. Nothing here tries another way.

mod carry;
mod open;
mod step;
mod tls;
mod types;
pub mod wait;

pub use types::{Exchange, Poll, Stage};
