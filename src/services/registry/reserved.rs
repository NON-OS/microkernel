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

//! Reserved core-service names and ports.

const RESERVED_NAMES: [&str; 5] =
    ["keyring", "entropy_pool", "crypto_pool", "vfs_pool", "market.index"];

/// True if `name`/`port` belongs to a trusted core service and so must not be
/// registrable through the runtime service-register syscall.
pub(crate) fn is_reserved_service(name: &str, port: u32) -> bool {
    RESERVED_NAMES.contains(&name)
        // Core service + reply ports (keyring..market occupy 4098..=4107).
        || (4098..=4107).contains(&port)
}

/// The only names a capsule may claim at runtime that it does not already
/// hold.
const RUNTIME_REGISTRABLE: [&str; 5] =
    ["net.tcp", "net.udp", "net.dhcp.client", "net.dns", "net.ip"];

pub(crate) fn is_runtime_registrable(name: &str) -> bool {
    RUNTIME_REGISTRABLE.contains(&name)
}
