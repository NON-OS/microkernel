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
//! What a RECEIPT job does with one frame, decided from the frame alone so
//! the rule is proved on the host: only its one caller is answered; the first
//! RECEIPT makes the one receipt the job ever makes, the same RECEIPT again is
//! answered with it again (a reply lost on the way is asked for once more),
//! and any other is EBUSY; RECEIPT_ACK drops the record only for the seal
//! number of the receipt it made, and anything else drops nothing.

use nonos_proof_abi::wire::{ack_seal_no, RECEIPT, RECEIPT_ACK};

pub const EBUSY: i32 = -16;
pub const EINVAL: i32 = -22;
pub const ENOMEM: i32 = -12;
pub const EPERM: i32 = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Answer this status and change nothing.
    Refuse(i32),
    /// Make the receipt from the frame's body and answer with it.
    Make,
    /// Answer with the receipt already made, unchanged.
    Again,
    /// Drop the record the made receipt is of, answer, and exit.
    Drop,
}

/// The receipt a job made: its record's seal number and the RECEIPT body
/// it was made for.
#[derive(Debug, Clone, Copy)]
pub struct Made<'a> {
    pub seal_no: u64,
    pub asked: &'a [u8],
}

pub fn step(from_caller: bool, op: u16, body: &[u8], heap: bool, made: Option<Made<'_>>) -> Step {
    match (from_caller, op, made) {
        (false, ..) => Step::Refuse(EPERM),
        (true, RECEIPT, Some(m)) if m.asked == body => Step::Again,
        (true, RECEIPT, Some(_)) => Step::Refuse(EBUSY),
        (true, RECEIPT, None) if !heap => Step::Refuse(ENOMEM),
        (true, RECEIPT, None) => Step::Make,
        (true, RECEIPT_ACK, Some(m)) if ack_seal_no(body) == Some(m.seal_no) => Step::Drop,
        _ => Step::Refuse(EINVAL),
    }
}
