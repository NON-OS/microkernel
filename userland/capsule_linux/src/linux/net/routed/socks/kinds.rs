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

//! What a SOCKS request comes to.

use alloc::vec::Vec;

/// The longest request taken: a SOCKS5 greeting is at most 257 bytes and
/// its request 262, a SOCKS4a request a few bytes and a name more. A guest
/// that has sent this much and no whole request is refused.
pub const REQUEST_MAX: usize = 512;

/// The longest name a CONNECT carries, as SOCKS5's one length byte allows.
pub const NAME_MAX: usize = 255;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ver {
    Four,
    Five,
}

/// Where a CONNECT asks to go, unresolved.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum To {
    V4([u8; 4]),
    Name(Vec<u8>),
}

/// A whole CONNECT.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Connect {
    pub to: To,
    pub port: u16,
}

/// What a conversation has heard.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Heard {
    /// No whole request yet.
    More,
    Connect(Ver, Connect),
    /// Refused or not SOCKS: what it was told is said, and it is closed.
    Closed,
}

/// A name a CONNECT may carry: 1 to 255 printable bytes, none of them the
/// ':' that would end its part of "host:port" at either network's front.
pub fn name_ok(name: &[u8]) -> bool {
    (1..=NAME_MAX).contains(&name.len()) && name.iter().all(|&b| b.is_ascii_graphic() && b != b':')
}
