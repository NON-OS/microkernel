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

//! The receipt's two halves from the kernel: the registry as MkAttestEntries
//! lists it, and the attestation doc for the receipt's challenge, cleared
//! (agentproof_receipt doc_clear.rs). The doc's quote binds the registry by
//! its root, so the entries a receipt carries must be the ones the kernel
//! folded: they are read before the doc and again after it, and a registry
//! that changed between is EAGAIN, asked again, never a receipt that fails.

use alloc::vec;
use alloc::vec::Vec;

use agentproof_receipt::napr::{challenge, doc_clear, DOC_MAX, ENTRIES_MAX};
use nonos_libc::{mk_attest_doc, mk_attest_entries};

const EINVAL: i32 = -22;
const EAGAIN: i32 = -11;

pub fn registry() -> Result<Vec<u8>, i32> {
    let mut regs = vec![0u8; ENTRIES_MAX];
    let n = mk_attest_entries(&mut regs);
    let n = usize::try_from(n).map_err(|_| n as i32)?;
    regs.truncate(n);
    (n > 0).then_some(regs).ok_or(EINVAL)
}

/// The cleared doc for `bound`'s challenge, made while the registry stayed
/// `entries`.
pub fn doc_for(bound: &[u8], entries: &[u8]) -> Result<Vec<u8>, i32> {
    let mut doc = vec![0u8; DOC_MAX];
    let n = mk_attest_doc(&challenge(bound), &mut doc);
    let doc = usize::try_from(n).ok().and_then(|n| doc_clear::clear(doc.get(..n)?));
    let doc = doc.ok_or(if n < 0 { n as i32 } else { EINVAL })?;
    match registry()? == entries {
        true => Ok(doc),
        false => Err(EAGAIN),
    }
}
