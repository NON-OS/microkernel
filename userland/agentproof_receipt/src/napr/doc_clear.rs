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
//! NAPR v0, the receipt of one boxed run (letter agent-02). Big-endian; after
//! The doc as a receipt carries it, cleared by layout alone, for the writer
//! in a capsule that does not carry nsh_attest: the kernel's NONOSATT v5
//! document with its capsule count zero and its event log empty, the two
//! parts its quote does not sign. The verifier's own reader (doc_form.rs)
//! checks the result, and a proof holds the two to the same bytes.

use alloc::vec::Vec;

/// The magic, version, challenge and registry root come before the count.
pub const COUNT_AT: usize = 8 + 4 + 32 + 32;
/// The PCRs a v5 doc carries, as nsh_attest's QUOTED_PCRS_V4 lists them.
pub const PCRS: usize = 8;
/// The count, registry and DMA fields, the PCR count, each PCR, then the log.
pub const LOG_AT: usize = COUNT_AT + 4 + 1 + 1 + 1 + 4 + 4 + 1 + PCRS * 33;
const MAGIC: &[u8; 8] = b"NONOSATT";
const VERSION: u32 = 5;

/// `kernel_doc` with the count zero and the log empty; None when its head
/// is not a v5 doc's or its log runs past its end.
pub fn clear(kernel_doc: &[u8]) -> Option<Vec<u8>> {
    if kernel_doc.get(..8)? != MAGIC || kernel_doc.get(8..12)? != VERSION.to_be_bytes() {
        return None;
    }
    let len = kernel_doc.get(LOG_AT..LOG_AT + 4)?;
    let log = u32::from_be_bytes([len[0], len[1], len[2], len[3]]) as usize;
    let rest = kernel_doc.get(LOG_AT.checked_add(4)?.checked_add(log)?..)?;
    let mut out = kernel_doc[..LOG_AT].to_vec();
    out[COUNT_AT..COUNT_AT + 4].fill(0);
    out.extend_from_slice(&0u32.to_be_bytes());
    out.extend_from_slice(rest);
    Some(out)
}
