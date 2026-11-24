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

//! The service a typed tool runs: `tool.<handle>`. A `linux --box` naming a
//! destination runs as `tool.linux` like any other `linux` run; its family
//! is granted the anonymity network by the front it uses, on the box the
//! kernel kept in its argv (userland/linux_net_grant).

use alloc::vec::Vec;

pub fn service(handle: &[u8]) -> Vec<u8> {
    [&b"tool."[..], handle].concat()
}
