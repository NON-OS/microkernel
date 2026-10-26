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

//! The route card's words: what the system is set to, a transport that has
//! said nothing, and three pieces joined into a fixed buffer.

pub(super) fn set_to(chosen: Option<u8>) -> &'static [u8] {
    match chosen {
        Some(nonos_route_proof::ROUTE_NYM) => b"this system's traffic is set to the Nym mixnet",
        Some(nonos_route_proof::ROUTE_ANYONE) => {
            b"this system's traffic is set to the Anyone network"
        }
        Some(nonos_route_proof::ROUTE_DIRECT) => b"this system's traffic is set to go direct",
        _ => b"the route this system is set to could not be read",
    }
}

pub(super) fn silent(board: bool) -> &'static [u8] {
    if board {
        b"has not reported"
    } else {
        b"route board not reachable"
    }
}

pub(super) fn join3<const N: usize>(out: &mut [u8; N], a: &[u8], b: &[u8], c: &[u8]) -> usize {
    let mut n = 0;
    for part in [a, b, c] {
        for &x in part {
            if n == N {
                return n;
            }
            out[n] = x;
            n += 1;
        }
    }
    n
}
