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

//! What the reader is told after a `proxy` command in the address bar.
//!
//! The answer went to the status line, which is drawn only while there is
//! no page: with a page on screen a mistyped proxy was dropped without a
//! word, and a proxy set while an anonymous network was chosen, where it
//! carries nothing, looked as if it had been taken into use.

use alloc::format;
use alloc::string::String;

use crate::browser::net::mixnet::Network;

/// What a `proxy` command did.
pub enum Outcome<'a> {
    Off,
    Set(&'a str, u16),
    Bad,
}

/// The line the reader is shown for `outcome` with `chosen` the network.
pub fn said(outcome: Outcome, chosen: Network) -> String {
    match (outcome, chosen) {
        (Outcome::Off, _) => String::from("SOCKS5 proxy off."),
        (Outcome::Set(host, port), Network::Direct) => {
            format!("Direct requests now go through the SOCKS5 proxy {host}:{port}.")
        }
        (Outcome::Set(host, port), net) => format!(
            "SOCKS5 proxy {host}:{port} kept for the direct network; requests now leave through {}.",
            net.label()
        ),
        (Outcome::Bad, _) => {
            String::from("Not a proxy address: type proxy socks5://host:port, or proxy off.")
        }
    }
}
