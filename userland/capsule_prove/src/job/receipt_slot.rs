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
//! Which record a receipt is of: the slot-0 note capsule_linux makes for this
//! app in this box (seal_slot.rs). The personality's own BusyBox, asked for
//! by the default policy of program zero, has no app_id of its own.

use agentproof_policy::boxed::hash::box_hash;
use agentproof_policy::boxed::preset::parse;
use agentproof_receipt::seal_slot::slot0;
use nonos_linux_policy::{app_id, decode, policy_hash, unlisted};
use nonos_proof_abi::wire::ReceiptRequest;

/// The slot-0 note this app in this box has, as capsule_linux makes it.
pub fn slot_of(req: &ReceiptRequest<'_>) -> Option<[u8; 32]> {
    let policy = decode(req.policy).ok()?;
    let app = match *req.policy == unlisted([0; 32]).encode() {
        true => [0; 32],
        false => app_id(&policy.program_blake3, &policy_hash(req.policy)),
    };
    slot0(&app, Some(&box_hash(&parse(req.box_word).ok()?)))
}
