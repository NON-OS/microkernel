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

/*
 * Whether a publisher call landed, from landed_read.rs's view answer (the
 * raw words, read strictly). allowance: one uint256. bondOf: payee,
 * publisherKey, amount net of slashes, unlockAt, slashed. deliveryAgentOf: one address. releaseOf:
 * releaseHash, bondId, publishedAt, withdrawn. anchoredAt: one uint64.
 * credit: one uint256. Any other count or shape is not landed.
 */

use super::abi::{uint, word_u64};
use super::call::Publisher;
use super::decode_words::{address, amount};

pub fn landed(p: &Publisher, words: &[[u8; 32]]) -> bool {
    use Publisher::*;
    let set = |t: &[u8; 32]| word_u64(t).is_some_and(|t| t != 0);
    match (p, words) {
        (Approve { amount: a, .. }, [w]) => *w == uint(*a),
        (Bond { publisher_key, amount: a }, [payee, key, held, _, _]) => {
            let ours = address(payee).is_some_and(|x| x != [0; 20]) && key == publisher_key;
            ours && amount(held).is_some_and(|h| h >= *a)
        }
        (SetDeliveryAgent { agent, .. }, [w]) => address(w) == Some(*agent),
        (RequestUnbond { .. }, [_, _, _, unlock, _]) => set(unlock),
        (WithdrawBond { .. }, [_, _, held, _, _]) => *held == [0; 32],
        (Publish { release_hash, bond_id, .. }, [hash, bond, at, gone]) => {
            hash == release_hash && bond == bond_id && set(at) && *gone == [0; 32]
        }
        (WithdrawRelease { .. }, [_, _, _, gone]) => *gone == uint(1),
        (Anchor { .. }, [at]) => set(at),
        (WithdrawBondCredit { .. }, [credit]) => *credit == [0; 32],
        _ => false,
    }
}
