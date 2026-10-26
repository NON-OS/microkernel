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

//! A headless run's receipt (letter agent-02), asked once the run has ended:
//! nonos.prove is asked for a RECEIPT job, sent the box and the policy the
//! report named, and answers with the NAPR v0 receipt. It is written to the
//! serial log, which is where a Gauntlet keeps it, and only once every row is
//! written is it acknowledged, so the record is dropped only once it is kept.
//! A receipt not kept is never acked, and its record stays sealed.

use alloc::string::String;
use alloc::vec;

use super::receipt_call::{call, job};
use super::receipt_frame::{receipt_body, seal_no, FRAME_MAX, RECEIPT, RECEIPT_ACK};

/// The receipt's seal number once it is kept and acknowledged, or why not.
pub fn ask(box_word: &[u8], policy: &[u8; 256]) -> Result<u64, String> {
    let body = receipt_body(box_word, policy).ok_or("the box word is too long")?;
    let port = job()?;
    let mut rx = vec![0u8; FRAME_MAX];
    let napr = call(port, RECEIPT, 1, &body, &mut rx)?.to_vec();
    let seal = seal_no(&napr).ok_or("the answer is not a receipt")?;
    if !super::receipt_log::keep(&napr, seal) {
        return Err("the receipt could not be written to the log; not acknowledged".into());
    }
    let rule = super::receipt_probe::before_ack(port, &body, &napr, &mut rx);
    call(port, RECEIPT_ACK, 2, &seal.to_le_bytes(), &mut rx)?;
    super::receipt_probe::after_ack(&body, &rule);
    Ok(seal)
}

/// The run's receipt asked for, and the line that says what came of it.
pub fn said(box_word: &[u8]) -> String {
    let Some(policy) = crate::jobs::box_app::last() else {
        return "[AGENTPROOF-RECEIPT] v=1 status=none why=no whole report\n".into();
    };
    match ask(box_word, &policy) {
        Ok(seal) => alloc::format!("[AGENTPROOF-RECEIPT] v=1 status=kept seal={seal:016x}\n"),
        Err(why) => alloc::format!("[AGENTPROOF-RECEIPT] v=1 status=none why={why}\n"),
    }
}
