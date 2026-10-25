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

//! What net.core says when smoltcp's DHCP client reports no lease
//! (`Event::Deconfigured`). smoltcp 0.11 starts its client with
//! `config_changed` set, so the first poll reports it while still
//! discovering, before any lease was held: a lease is said dropped only when
//! one was. Pure: net_core_proofs::deconfigured_line_tests.

/// A held lease ran out unrenewed, was NAKed, or was reset when the link
/// came back.
pub const DROPPED: &[u8] = b"[NET-CORE] lease dropped; DHCP asks for a new one\n";
/// No lease was held: the client is starting, or starting again.
pub const ASKING: &[u8] = b"[NET-CORE] DHCP asking for a lease\n";

/// The line for a Deconfigured, `held` saying whether a lease was held then.
pub const fn line(held: bool) -> &'static [u8] {
    if held {
        DROPPED
    } else {
        ASKING
    }
}
