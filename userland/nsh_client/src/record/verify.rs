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

//! `verify_record`: steps 1 to 6 of nsh-v1-record.md, in order, and every
//! checkpoint given. A record that parses whole must pass all six; one
//! that does not is read as a footerless record and is accepted only as
//! far as its last checkpoint, which must cover entries it holds.

use super::walk::walk;
use super::{bound, header, Bound, RecordRefusal, Report, Verdict};
use crate::ct::eq;
use crate::{hybrid, Attester, Policy};
use nonos_hash::sha512;
use nsh_crypto::{labelled, Label};
use nsh_wire::record::MAX_RECORD;
use nsh_wire::Record;

/// Check `record` against the attestation document `doc` and the fleet in
/// `policy`, with the checkpoints the host wrote beside it, and hold its
/// header to what `expect` names.
pub fn verify_record<A: Attester + ?Sized>(
    record: &[u8],
    doc: &[u8],
    ckpts: &[&[u8]],
    expect: &Bound,
    policy: &Policy,
    attester: &A,
) -> Result<Report, RecordRefusal> {
    let rec = match Record::parse(record) {
        Ok(rec) => rec,
        Err(e) if ckpts.is_empty() || record.len() > MAX_RECORD => {
            return Err(RecordRefusal::Parse(e))
        }
        Err(_) => return super::cut::cut_short(record, doc, ckpts, expect, policy, attester),
    };
    let h = &rec.header;
    bound::check(h, expect).map_err(RecordRefusal::Unbound)?;
    header::check(h, doc, policy, attester)?;
    let mut marks = super::cut::marks(ckpts, h)?;
    let w = walk(h.bytes, rec.entries(), &mut marks).map_err(RecordRefusal::Ckpt)?;
    if !eq(&w.chain, rec.footer.chain) {
        return Err(RecordRefusal::Chain);
    }
    let digest = labelled(Label::RecordSig, &sha512(rec.signed));
    hybrid::verify(h.host.ed, h.host.ml, &digest, &rec.footer.sig).map_err(RecordRefusal::Sig)?;
    let f = &rec.footer;
    let boxed = w.first_box.is_some();
    let verdict =
        Verdict::Complete { entries: f.count, end_ms: f.end_ms, end_reason: f.end_reason, boxed };
    Ok(super::cut::report(h, verdict))
}
