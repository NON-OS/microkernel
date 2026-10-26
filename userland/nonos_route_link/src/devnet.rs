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

//! The local test chain's RPC (P5): one fixed name, under the reserved .test
//! domain, dialled directly at the development machine's host as QEMU's user
//! network shows it. Only a `devnet` build compiles this in, and only
//! development images are built so; every other name is left as it was.

/// The one name the local chain's RPC answers to; its certificate names it.
pub const DEVNET_RPC_HOST: &str = "rpc.devnet.nonos.test";
/// Where that name is dialled: QEMU user networking's host, a private address.
pub const DEVNET_RPC_AT: &str = "10.0.2.2";

/// Whether `host` is exactly the local chain's RPC name.
pub fn is_devnet_rpc(host: &str) -> bool {
    host == DEVNET_RPC_HOST
}

/// The address a direct connection to `host` dials.
pub fn dial_at(host: &str) -> &str {
    if is_devnet_rpc(host) {
        DEVNET_RPC_AT
    } else {
        host
    }
}
