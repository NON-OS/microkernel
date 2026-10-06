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

//! Which path a fetch takes. Everything goes over the mixnet, so what this
//! machine installs is not visible on the network it sits on, except a mirror
//! on a private or link-local address: a LAN or offline mirror, which a mixnet
//! exit could not reach. That exception is said in the log every time, and in
//! design/install-network.md.

/// 10/8, 172.16/12, 192.168/16 and 169.254/16, as a dotted quad.
pub fn is_local(ip: &str) -> bool {
    let mut q = [0u8; 4];
    let mut parts = ip.split('.');
    for slot in q.iter_mut() {
        match parts.next().and_then(|p| p.parse().ok()) {
            Some(v) => *slot = v,
            None => return false,
        }
    }
    if parts.next().is_some() {
        return false;
    }
    matches!(q, [10, ..] | [192, 168, ..] | [169, 254, ..])
        || (q[0] == 172 && (16..32).contains(&q[1]))
}
