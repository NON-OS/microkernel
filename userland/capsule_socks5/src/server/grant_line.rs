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

//! The one line a grant decision leaves on the serial log, `[SOCKS5] grant
//! <code>`: 0 for granted, else the refusal's number (linux_net_grant
//! refusal.rs). It takes the code alone, so no host, destination, argv or
//! measurement can reach the log through it.

pub fn said(code: u8) {
    super::trace::step(b"grant", u64::from(code));
}
