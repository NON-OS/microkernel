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

//! What a command that leaves directly says when this machine has no network.
//!
//! With no cable and no Wi-Fi joined, `ping example.com` waited three
//! seconds on the window's thread while the resolver resent its question to
//! nobody, then said "dns lookup failed (servfail)"; `nslookup` said "not
//! resolved". Neither says what is missing. An address comes only from DHCP,
//! so before anything is sent the DHCP client is asked whether it holds a
//! lease, and a machine without one is told so at once.

use alloc::vec::Vec;

/// What the DHCP client said about its lease.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LeaseSeen {
    /// No DHCP client is registered on this boot.
    NoClient,
    /// The client answered: it holds no address.
    Unbound,
    /// The client answered with an address.
    Bound,
    /// The client did not answer in time (busy getting an address, say):
    /// nothing is known, so the command goes ahead and says what it finds.
    Silent,
}

/// Why nothing is sent, said as `command: why`, or None when it may go ahead.
pub fn offline_line(command: &[u8], seen: LeaseSeen) -> Option<Vec<u8>> {
    let why: &[u8] = match seen {
        LeaseSeen::Bound | LeaseSeen::Silent => return None,
        LeaseSeen::NoClient => b"no network is running on this boot, so nothing was sent",
        LeaseSeen::Unbound => {
            b"not connected to a network (no address yet). Plug in a cable or join a \
              Wi-Fi network in Settings, then try again; nothing was sent"
        }
    };
    let mut line = Vec::with_capacity(command.len() + 2 + why.len());
    line.extend_from_slice(command);
    line.extend_from_slice(b": ");
    line.extend_from_slice(why);
    Some(line)
}
