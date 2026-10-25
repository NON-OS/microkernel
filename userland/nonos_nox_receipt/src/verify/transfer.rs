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

use super::topic::TRANSFER_TOPIC;
use super::types::Payment;
use crate::hex::{decode_fixed, decode_u128};
use crate::logs::RawLog;

/// If this log is a transfer of `token` to `payee` for at least `min_amount`,
/// return the payment; otherwise `None`.
pub(super) fn transfer_to(
    log: &RawLog<'_>,
    payee: &[u8; 20],
    min_amount: u128,
    token: &[u8; 20],
) -> Option<Payment> {
    // Emitted by the token itself, so a lookalike contract's event is ignored.
    let mut addr = [0u8; 20];
    if !decode_fixed(log.address, &mut addr) || &addr != token {
        return None;
    }
    // Transfer(from, to, value): topic0 signature, topic1 from, topic2 to.
    if log.topic_count < 3 {
        return None;
    }
    let mut topic0 = [0u8; 32];
    if !decode_fixed(log.topics[0], &mut topic0) || topic0 != TRANSFER_TOPIC {
        return None;
    }
    // Indexed address topics are a full 32-byte word, address in the low 20.
    let mut from_word = [0u8; 32];
    if !decode_fixed(log.topics[1], &mut from_word) {
        return None;
    }
    let mut to_word = [0u8; 32];
    if !decode_fixed(log.topics[2], &mut to_word) {
        return None;
    }
    if &to_word[12..32] != payee {
        return None;
    }
    let amount = decode_u128(log.data)?;
    if amount < min_amount {
        return None;
    }
    let mut from = [0u8; 20];
    from.copy_from_slice(&from_word[12..32]);
    Some(Payment { from, amount })
}
