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

//! The ops the keyring retired. 11 signed receipts and 12 built an approve,
//! both for the retired, upgradeable ReceiptSettlement proxy, which the OS no
//! longer controls: an allowance handed to it could be taken by whoever
//! upgrades it. 13 and 18 signed a plain ETH and a NOX transfer for any
//! caller, and nothing in the OS asked for either. All four answer EINVAL,
//! and their numbers stay taken so no new op reuses them.

use alloc::vec::Vec;

use super::encode::encode_response;
use super::errno::EINVAL;
use super::types::{Request, OP_SIGN_ETH_TRANSFER, OP_SIGN_NOX_TRANSFER};
use super::types::{OP_SIGN_NOX_APPROVE, OP_SIGN_NOX_RECEIPT};

pub const RETIRED: [u16; 4] =
    [OP_SIGN_NOX_RECEIPT, OP_SIGN_NOX_APPROVE, OP_SIGN_ETH_TRANSFER, OP_SIGN_NOX_TRANSFER];

/// The answer to a retired op, or None for any other.
pub fn retired(req: &Request<'_>) -> Option<Vec<u8>> {
    RETIRED.contains(&req.op).then(|| encode_response(req.seq, EINVAL, &[]))
}
