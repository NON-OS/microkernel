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

//! The first word of every derived id: keccak256 of a versioned label, inside
//! abi.encode, never the raw string. Each id lives in its own space, so a
//! channel id can never be passed off as an order or a bond.
//!
//!   channelId = keccak256(abi.encode(CHANNEL_TAG, chainId, settlement, payer, payee,
//!                                    listingId, openNonce))
//!   orderId   = keccak256(abi.encode(ORDER_TAG, chainId, escrow, terms))
//!   bondId    = keccak256(abi.encode(BOND_TAG, payee, publisherKey))
//!   listingId = keccak256(abi.encode(LISTING_TAG, publisherKey, keccak256(appId)))
//!   releaseId = keccak256(abi.encode(RELEASE_TAG, listingId, keccak256(version)))
//!   refundCommit = keccak256(abi.encode(REFUND_TAG, refundTo, refundSalt))

use crate::hex::hex32;

pub const CHANNEL_TAG: [u8; 32] =
    hex32("f95e9f681f7516c5560dd2a92b8923ef2f81c398b248fe4a198e639a4c29c5dd");
pub const ORDER_TAG: [u8; 32] =
    hex32("594b8b439878213a0e9d38906a542156dc8ba5805d7372e6ebe29d4fc2f057da");
pub const BOND_TAG: [u8; 32] =
    hex32("93653ab7449c54fb76d207fd75b8138d1c1adda4f0e20b1fb4a5e0b50f7acfe9");
pub const LISTING_TAG: [u8; 32] =
    hex32("fc28c341928084c4d621b5d5ca20ad5fe5f9626b525ba55ca86203e2b12545b4");
pub const RELEASE_TAG: [u8; 32] =
    hex32("77721460c4979894ffb77be556e591c2530f4f8cf95f1677009b6b0659ff9519");
pub const REFUND_TAG: [u8; 32] =
    hex32("f23dc5c999ebe275791128a480cc83d5d50ebc78da4c4e0cff83cd63f76d2524");

/// Each tag beside the label it hashes, for the tests and for anyone checking.
pub const TAGS: [(&str, [u8; 32]); 6] = [
    ("NONOS.nox.channel.v1", CHANNEL_TAG),
    ("NONOS.nox.order.v1", ORDER_TAG),
    ("NONOS.nox.bond.v1", BOND_TAG),
    ("NONOS.nox.listing.v1", LISTING_TAG),
    ("NONOS.nox.release.v1", RELEASE_TAG),
    ("NONOS.nox.refund.v1", REFUND_TAG),
];
