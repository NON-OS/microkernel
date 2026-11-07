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

//! An ICMP message leaves in the clear, so net.ip sends one only on the
//! Direct route, as net.dns resolves only there. The terminal's ping and an
//! nsh session's both check the route before they ask; this holds it for
//! every capsule that can reach net.ip.

use nonos_policy_proto::route;

/// `chosen` is the NetworkRoute the policy store gave.
pub fn clear_send_allowed(chosen: Option<u8>) -> bool {
    chosen == Some(route::DIRECT)
}
