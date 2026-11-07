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

//! Who may open a listener: nsh.front alone, by the pid the service registry
//! names for it. net.core takes a listen only from net.sockets, so this is
//! where the listening app is chosen; without it any capsule holding Network
//! could take connections from outside any mixnet while Direct is chosen.
//! Release and production kernels refuse nsh.front, so no listener opens
//! there. Pure, so the host proofs hold it.

/// `nsh_front` is nsh.front's pid as the service registry gave it.
pub fn listener_allowed(sender_pid: u32, nsh_front: Option<u32>) -> bool {
    nsh_front.is_some_and(|pid| pid != 0 && pid == sender_pid)
}
