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
//! The run's half: the record is a finished seal by a trusted supervisor,
//! and its slot 0 is this app held in this box.

use agentproof_policy::boxed::hash::box_hash;
use agentproof_policy::boxed::preset::{parse, BoxPolicy};
use nonos_linux_policy::{app_id, decode, policy_hash, unlisted};
use nonos_proof_abi::{decode_record, Record, FLAG_FINAL, FLAG_OVERFLOW, FLAG_SATURATED};

use super::Refusal;
use crate::napr::Receipt;
use crate::seal_slot::slot0;
use crate::trust::Trust;

pub struct Run {
    pub record: Record,
    pub policy: BoxPolicy,
    pub box_hash: [u8; 32],
    /// The program, or None for the personality's own BusyBox, whose note
    /// names none.
    pub program: Option<[u8; 32]>,
    pub app_id: [u8; 32],
    pub built_in: bool,
}

pub fn run(r: &Receipt<'_>, trust: &Trust) -> Result<Run, Refusal> {
    let record = decode_record(r.record).ok_or(Refusal::Record)?;
    let finished = record.flags & FLAG_FINAL != 0;
    if !finished || record.flags & (FLAG_SATURATED | FLAG_OVERFLOW) != 0 {
        return Err(Refusal::Unfinished);
    }
    let known =
        trust.supervisors.iter().any(|s| s.meas == record.sup_meas && s.caps == record.sup_caps);
    if record.authority != 0 || !known {
        return Err(Refusal::Supervisor);
    }
    let policy = parse(r.box_word).map_err(|_| Refusal::Box)?;
    let box_hash = box_hash(&policy);
    let app = decode(r.policy).map_err(|_| Refusal::Policy)?;
    let program = app.program_blake3;
    let app_id = app_id(&program, &policy_hash(r.policy));
    /*
     * The personality's own BusyBox has no app_id, so its note binds only the
     * box: such a receipt names no program, and carries the one policy that
     * says so, the default box of program zero.
     */
    let none = unlisted([0; 32]).encode();
    let built_in = match record.app_id {
        a if Some(a) == slot0(&app_id, Some(&box_hash)) && *r.policy != none => false,
        a if Some(a) == slot0(&[0; 32], Some(&box_hash)) && *r.policy == none => true,
        _ => return Err(Refusal::Binding),
    };
    let program = (!built_in).then_some(program);
    Ok(Run { record, policy, box_hash, program, app_id, built_in })
}
