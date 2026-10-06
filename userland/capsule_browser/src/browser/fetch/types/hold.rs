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

//! A fetch through a proxy that waits to ask again (`socks::hold`).

/// Why it waits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Wait {
    /// The network has not connected yet: the proxy refused the CONNECT
    /// with "network unreachable" (3), as both do while they come up.
    NotYet,
    /// The proxy is serving as many conversations as it can and turned the
    /// greeting away.
    Full,
}

/// Why, and since when: the first refusal, which bounds the whole wait.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Hold {
    pub why: Wait,
    pub since: i64,
}
