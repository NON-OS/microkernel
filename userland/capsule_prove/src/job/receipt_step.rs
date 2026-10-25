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
//! the rule is proved on the host: only its one caller is answered; RECEIPT
//! makes the receipt, again each time; RECEIPT_ACK drops the record only for
//! the seal number of the receipt it made, and anything else drops nothing.

use nonos_proof_abi::wire::{ack_seal_no, RECEIPT, RECEIPT_ACK};

pub const EINVAL: i32 = -22;
pub const ENOMEM: i32 = -12;
pub const EPERM: i32 = -1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Answer this status and change nothing.
    Refuse(i32),
    /// Make the receipt from the frame's body and answer with it.
    Make,
    /// Drop the record the made receipt is of, answer, and exit.
    Drop,
}

/// `made`: the seal number of the receipt this job last answered, if any.
pub fn step(from_caller: bool, op: u16, body: &[u8], heap: bool, made: Option<u64>) -> Step {
    match (from_caller, op) {
        (false, _) => Step::Refuse(EPERM),
        (true, RECEIPT) if !heap => Step::Refuse(ENOMEM),
        (true, RECEIPT) => Step::Make,
        (true, RECEIPT_ACK) if made.is_some() && ack_seal_no(body) == made => Step::Drop,
        _ => Step::Refuse(EINVAL),
    }
}
