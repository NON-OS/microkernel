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

//! The meter's two files, under /data because that is the tree the block
//! store keeps. Both are fixed length: the store replaces a kept record only
//! with one of the same length.

/// The meter's state, tagged (`nonos_nox_meter::persist`).
pub const STATE_PATH: &[u8] = b"/data/meter.state";
/// The meter's secp256k1 secret, sealed to this machine under its bound root.
pub const KEY_PATH: &[u8] = b"/data/meter.key";
/// The meter's escrow orders, sealed: each holds a refund salt.
pub const ORDERS_PATH: &[u8] = b"/data/meter.orders";
pub(super) const DIR: &[u8] = b"/data";
