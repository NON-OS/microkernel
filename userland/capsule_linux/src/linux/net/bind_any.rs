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

//! A client's bind to 0.0.0.0:0, as musl's resolver and BusyBox's nc make
//! before they send or connect. It names no address and no port, so for a
//! program that may reach out it is answered as done and changes nothing:
//! its lookup and its connect then go on as an unbound socket's would. Any
//! other bind is bind.rs's, held to NET_LISTEN_LO and to 127.0.0.0/8.

/// Whether a bind to `ip`:`port` is that unnamed bind, by a program that
/// `reaches_out` (holds NET_ANON).
pub fn unnamed(ip: [u8; 4], port: u16, reaches_out: bool) -> bool {
    ip == [0; 4] && port == 0 && reaches_out
}
