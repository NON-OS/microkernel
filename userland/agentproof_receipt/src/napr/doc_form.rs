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
//! The doc as a receipt carries it: the kernel's NONOSATT v5 document with
//! its three parts the quote does not sign cleared: the capsule count, the
//! registry's complete flag and the firmware's event log. nsh checks those
//! against registry entries and an enrolment certificate a receipt does not
//! carry; left in, they would be bytes anyone could change without the
//! receipt failing, so a receipt claims nothing of them.

use alloc::vec::Vec;

use nsh_attest::consts::PCR_COUNT;
use nsh_attest::Doc;

/// Magic, version, challenge and registry root come before the count.
const COUNT_AT: usize = 8 + 4 + 32 + 32;
/// The registry's complete flag, after the count.
const COMPLETE_AT: usize = COUNT_AT + 4;
/// The count, the registry and DMA fields, the PCR count, then each PCR.
const LOG_AT: usize = COUNT_AT + 4 + 1 + 1 + 1 + 4 + 4 + 1 + PCR_COUNT * 33;

/// `kernel_doc` with the count zero, the complete flag clear and the log
/// empty; None for bytes that are not one v5 document.
pub fn receipt_doc(kernel_doc: &[u8]) -> Option<Vec<u8>> {
    let log = Doc::parse(kernel_doc).ok()?.log.len();
    let mut out = kernel_doc.get(..LOG_AT)?.to_vec();
    out[COUNT_AT..COUNT_AT + 4].fill(0);
    out[COMPLETE_AT] = 0;
    out.extend_from_slice(&0u32.to_be_bytes());
    out.extend_from_slice(kernel_doc.get(LOG_AT + 4 + log..)?);
    let d = Doc::parse(&out).ok()?;
    cleared(&d).then_some(out)
}

/// Whether a doc is in the form a receipt carries.
pub fn cleared(doc: &Doc<'_>) -> bool {
    doc.capsule_count == 0 && !doc.registry_complete && doc.log.is_empty()
}
