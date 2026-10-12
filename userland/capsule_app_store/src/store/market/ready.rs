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

//! Why a listing can or cannot be installed.

use nonos_market_proto::{parse_readiness, ready_body, OP_INSTALL_READY};
use nonos_nox_config::SEPOLIA;

pub use nonos_market_proto::{Readiness, GATES};

use super::failure::Failure;
use super::wire::exchange;

/// The release is left unnamed: the capsule resolves the default then,
/// the same way `get_release` does, so the two describe one release. The
/// bond is read on Sepolia, the one chain the contracts are on (v1.4.23).
pub fn fetch(port: u32, request_id: u32, listing: &[u8]) -> Result<Readiness, Failure> {
    let body = exchange(
        port,
        OP_INSTALL_READY,
        request_id,
        &ready_body(listing, b"", SEPOLIA).ok_or(Failure::Malformed)?,
    )?;
    parse_readiness(&body).ok_or(Failure::Malformed)
}
