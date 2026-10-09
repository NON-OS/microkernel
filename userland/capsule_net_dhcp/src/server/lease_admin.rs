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

//! Who may change the machine's DHCP lease.
//!
//! net.dhcp takes its lease by itself at boot. OP_LEASE_REQUEST runs a
//! whole exchange with the network's server, OP_LEASE_RENEW asks it to
//! extend the lease and OP_LEASE_RELEASE gives the address up, which takes
//! the machine off the network. None of them checked the sender, so any
//! capsule that could reach net.dhcp could drop the machine's address, or
//! have the machine broadcast its MAC in a fresh exchange as often as it
//! liked. Only a service named here may now, and no service in the tree
//! sends them today: the Settings app is where a person would ask. Anyone
//! else gets E_PERM. OP_LEASE_STATUS stays open to every client.

/// The services that may request, renew or release the lease.
pub const LEASE_ADMINS: &[&[u8]] = &[b"app.settings"];

/// Whether `sender_pid` is one of LEASE_ADMINS, by the pid `lookup` (the
/// service registry) names for each.
pub fn may_change_lease(sender_pid: u32, lookup: impl Fn(&[u8]) -> Option<u32>) -> bool {
    sender_pid != 0 && LEASE_ADMINS.iter().any(|name| lookup(name) == Some(sender_pid))
}
