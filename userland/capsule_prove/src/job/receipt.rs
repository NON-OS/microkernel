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

use agentproof_receipt::napr::{challenge, doc_clear, write, DOC_MAX};
use nonos_libc::{mk_attest_doc, mk_seal_peek};
use nonos_proof_abi::wire::ReceiptRequest;
use nonos_proof_abi::{decode_record, FLAG_FINAL, FLAG_OVERFLOW, FLAG_SATURATED, RECORD_BYTES};

use crate::assemble::wipe_bytes;
use crate::market::WITNESS_BYTES;

const EINVAL: i32 = -22;
const LIST_MAX: usize = 4096;

pub struct Receipt {
    pub napr: Vec<u8>,
    pub slot: [u8; 32],
    pub seal_no: u64,
}

pub fn make(req: &ReceiptRequest<'_>) -> Result<Receipt, i32> {
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
    let bound = write::bound(&record, req.box_word, req.policy, &[0; 32]);
    let mut doc = vec![0u8; DOC_MAX];
    let n = mk_attest_doc(&challenge(&bound), &mut doc);
    let doc = usize::try_from(n).ok().and_then(|n| doc_clear::clear(doc.get(..n)?));
    let napr = write::finish(bound, &doc.ok_or(if n < 0 { n as i32 } else { EINVAL })?);
    Ok(Receipt { napr, slot, seal_no: rec.seal_no })
}
