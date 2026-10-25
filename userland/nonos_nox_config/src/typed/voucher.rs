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

//! The pay-per-use voucher (plan/20). The meter signs one per charge with a
//! rising `cumulative`; the payee settles only the newest, and the contract
//! pays the difference from what it already settled.

use super::TypedData;
use crate::domain::SETTLEMENT;
use crate::hex::hex32;

pub const VOUCHER: TypedData = TypedData {
    type_string: "Voucher(bytes32 channelId,address payer,address payee,bytes32 listingId,\
bytes32 unit,uint256 cumulative,uint64 seq,uint64 expiry,bytes32 usageRoot)",
    typehash: hex32("6481006cfbd6bb98804dafc05500eb2c23dbdb5206a1d046cdb9b2621c2780fe"),
    domain: &SETTLEMENT,
    fields: 9,
};
