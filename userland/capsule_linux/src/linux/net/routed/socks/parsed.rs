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

//! What the bytes at the front of a request came to.

use super::reply::{ADDRESS_TYPE_NOT_SUPPORTED, COMMAND_NOT_SUPPORTED, GENERAL_FAILURE};

/// Why a request in a version this endpoint speaks is answered with a
/// refusal and no stream.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// BIND or UDP ASSOCIATE, or any command but CONNECT.
    Command,
    /// IPv6, or any address type but IPv4 and a name.
    AddressType,
    /// An empty or unprintable name, a port of 0, or the address 0.0.0.0.
    Malformed,
}

/// What the bytes at the front of a buffer came to.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Parsed<T> {
    /// Not all of it has come.
    Need,
    /// It, and how many bytes it took.
    Done(T, usize),
    Refused(Refusal),
    /// Not this version at all: closed without a reply.
    Bad,
}

impl Refusal {
    /// The SOCKS5 reply code that refuses it.
    pub fn code(self) -> u8 {
        match self {
            Refusal::Command => COMMAND_NOT_SUPPORTED,
            Refusal::AddressType => ADDRESS_TYPE_NOT_SUPPORTED,
            Refusal::Malformed => GENERAL_FAILURE,
        }
    }
}
