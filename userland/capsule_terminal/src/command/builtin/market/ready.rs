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

//! INSTALL_READY for the listing's default release, its bond read on
//! Sepolia, the one chain the contracts are on (v1.4.23).

use nonos_market_proto::{parse_readiness, ready_body, Readiness, OP_INSTALL_READY};
use nonos_nox_config::SEPOLIA;

use super::call::call;
use super::failure::Failure;

pub(super) fn readiness(id: &[u8]) -> Result<Option<Readiness>, Failure> {
    call(OP_INSTALL_READY, &ready_body(id, b"", SEPOLIA)).map(|b| parse_readiness(&b))
}
