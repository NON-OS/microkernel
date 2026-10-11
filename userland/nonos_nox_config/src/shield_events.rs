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

//! The Shield pool's and its AmountPolicy's events, as declared in L1-Shield
//! 9d17ad9 (ShieldedPool.sol:190-202, AmountPolicy.sol:173-174), for log reads
//! by topic0 (v1.4.17). Each topic is pinned by a test against its signature.
//! IntentUnshielded is in base units, and is also emitted for fee-paying
//! transfers with recipient 0.

use crate::hex::hex32;

pub const INTENT_UNSHIELDED: [u8; 32] =
    hex32("c47b6c337802617269857ef446c4815968157c689136250997218c60ea512d17");
pub const PAYOUT_CREDITED: [u8; 32] =
    hex32("cbb28b75ab689cf3514e6bfdd08fbe8f677ec07df2353dca32e299aa58f31a48");
pub const PAYOUT_CLAIMED: [u8; 32] =
    hex32("36ec4f22247e910e64f31328f84eeb06eb17c77a840cd96304cd396824f1f5da");
pub const NOTE_COMMITTED: [u8; 32] =
    hex32("4d307fe1adf9127a7d28c596bdfd7feabb9b3ee98ae414268ca26f6429c4c7c7");
pub const POLICY_PAUSED: [u8; 32] =
    hex32("8a28e5c0320c98a7369c776140335d42630ce2d10b257d233aba6dd8313f4141");
pub const POLICY_UNPAUSED: [u8; 32] =
    hex32("5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa");

/// Each topic beside the signature it hashes.
pub const SHIELD_EVENTS: [(&str, [u8; 32]); 6] = [
    ("IntentUnshielded(uint64,address,uint256,uint256)", INTENT_UNSHIELDED),
    ("PayoutCredited(uint64,address,uint256)", PAYOUT_CREDITED),
    ("PayoutClaimed(uint64,address,address,uint256)", PAYOUT_CLAIMED),
    ("NoteCommitted(bytes32,uint40)", NOTE_COMMITTED),
    ("Paused(address,uint64)", POLICY_PAUSED),
    ("Unpaused(address)", POLICY_UNPAUSED),
];
