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
//! What one address holds (v1.4.55, v1.4.56): NOX from balanceOf, then ETH
//! from GAS, two requests spaced apart for the same address. Pure.

use alloc::format;
use alloc::string::String;

/// NOX and ETH at one address, in wei, once both reads answered.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Held {
    pub nox: u128,
    pub eth: u128,
}

impl Held {
    pub fn any(&self) -> bool {
        self.nox > 0 || self.eth > 0
    }
}

/// Both reads, or None when either did not answer.
pub fn both(nox: Option<u128>, eth: Option<u128>) -> Option<Held> {
    Some(Held { nox: nox?, eth: eth? })
}

/// The address being read: NOX first, then ETH.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reading {
    pub address: [u8; 20],
    /// For the scan, or for the due list.
    pub scan: bool,
    /// balanceOf's answer once it came (None inside: it did not); GAS is next.
    pub nox: Option<Option<u128>>,
    /// This phase's request is out.
    pub asked: bool,
}

/// One row: the NOX and the ETH held there, each written by `wei` if any.
pub fn line(address: &[u8; 20], held: Held, wei: impl Fn(u128) -> String) -> String {
    let short: String = address.iter().take(4).map(|b| format!("{b:02x}")).collect();
    let text = match (held.nox, held.eth) {
        (0, 0) => String::from("nothing"),
        (n, 0) => format!("{} NOX", wei(n)),
        (0, e) => format!("{} ETH", wei(e)),
        (n, e) => format!("{} NOX and {} ETH", wei(n), wei(e)),
    };
    format!("0x{short}…: {text}, a refund or a reclaim")
}
