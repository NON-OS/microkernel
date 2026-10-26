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
//! The receipt of one boxed run (letter agent-02): the record its slot 0
//! names, peeked and left sealed; the attestation doc for the receipt's own
//! challenge; the NAPR v0 bytes. The record is dropped by receipt_serve.rs
//! only once the caller acknowledges it, so nothing is lost before then.

use alloc::vec;
use alloc::vec::Vec;

use agentproof_receipt::napr::write;
use nonos_libc::mk_seal_peek;
use nonos_proof_abi::wire::{ReceiptRequest, GET_MAX};
use nonos_proof_abi::{decode_record, FLAG_FINAL, FLAG_OVERFLOW, FLAG_SATURATED, RECORD_BYTES};

use super::receipt_entry::{measure_of, ENTRY_LEN};
use crate::assemble::wipe_bytes;
use crate::market::WITNESS_BYTES;

const EINVAL: i32 = -22;
const E2BIG: i32 = -7;
const LIST_MAX: usize = 4096;
const _: () = assert!(nonos_libc::ATTEST_ENTRY_LEN == ENTRY_LEN);

/// `asked`: the RECEIPT body it was made for, the one it is answered again for.
pub struct Receipt {
    pub napr: Vec<u8>,
    pub asked: Vec<u8>,
    pub slot: [u8; 32],
    pub seal_no: u64,
}

/// The receipt a RECEIPT body asks for, its launcher `asker`, who asked.
pub fn make(body: &[u8], asker: u32) -> Result<Receipt, i32> {
    let req = &ReceiptRequest::decode(body).ok_or(EINVAL)?;
    let slot = super::receipt_slot::slot_of(req).ok_or(EINVAL)?;
    let mut buf = vec![0u8; RECORD_BYTES + WITNESS_BYTES + 32 * LIST_MAX];
    let n = mk_seal_peek(&slot, &mut buf);
    let record: Option<[u8; RECORD_BYTES]> =
        buf.get(..RECORD_BYTES).and_then(|r| r.try_into().ok());
    wipe_bytes(&mut buf);
    if n < RECORD_BYTES as i64 {
        return Err(if n < 0 { n as i32 } else { EINVAL });
    }
    let record = record.ok_or(EINVAL)?;
    let rec = decode_record(&record).ok_or(EINVAL)?;
    let finished = rec.flags & FLAG_FINAL != 0 && rec.flags & (FLAG_SATURATED | FLAG_OVERFLOW) == 0;
    if !finished || rec.authority != 0 {
        return Err(EINVAL);
    }
    let entries = super::receipt_doc::registry()?;
    let launcher = measure_of(&entries, asker).ok_or(EINVAL)?;
    let bound = write::bound(&record, req.box_word, req.policy, &launcher);
    let (doc, capsules) = super::receipt_doc::doc_for(&bound, &entries)?;
    let napr = write::finish(bound, &doc, &entries, &capsules);
    // One reply frame carries it, or nothing does.
    if napr.len() > GET_MAX {
        return Err(E2BIG);
    }
    Ok(Receipt { napr, asked: body.to_vec(), slot, seal_no: rec.seal_no })
}
