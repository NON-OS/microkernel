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

//! The answer a domain gives for one channel.

/// What the station may do on a channel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Permit {
    /// Nothing: not even a reply to an AP that is heard there.
    Disabled,
    /// Listen; transmit only to an access point heard on the channel, whose
    /// beacon is the enabling signal. `dfs` marks a radar channel.
    Passive { dfs: bool },
    /// Probe and join freely.
    Active,
}
