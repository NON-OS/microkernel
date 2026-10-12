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

//! One GHES or GHESv2 error source (ACPI 6.5, 18.3.2.7-8): the Generic
//! Address Structure that holds the address of its error status block, how
//! firmware tells the OS (the notification structure, 18.3.2.9), and for
//! v2 the Read Ack register the OS writes once it has read the block.

use super::le::{u32_at, u64_at};

/// Notification types (table 18.14) NONOS serves; the rest are listed.
pub const POLLED: u8 = 0;
pub const SCI: u8 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gas {
    pub space: u8,
    pub width: u8,
    pub address: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ack {
    pub register: Gas,
    pub preserve: u64,
    pub write: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ghes {
    /// Holds the 64-bit physical address of the error status block.
    pub status_address: Gas,
    pub notify: u8,
    /// For a polled source, how often, in milliseconds.
    pub poll_ms: u32,
    pub block_len: u32,
    pub ack: Option<Ack>,
}

fn gas(s: &[u8], at: usize) -> Gas {
    Gas { space: s[at], width: s[at + 1], address: u64_at(s, at + 4) }
}

/// `s` is the whole structure: 64 bytes, or 92 for v2.
pub(super) fn ghes(s: &[u8], v2: bool) -> Option<Ghes> {
    let need = if v2 { 92 } else { 64 };
    if s.len() < need {
        return None;
    }
    let ack =
        v2.then(|| Ack { register: gas(s, 64), preserve: u64_at(s, 76), write: u64_at(s, 84) });
    Some(Ghes {
        status_address: gas(s, 20),
        notify: s[32],
        poll_ms: u32_at(s, 36),
        block_len: u32_at(s, 60),
        ack,
    })
}
