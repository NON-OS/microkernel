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

//! A record without a footer, checked as far as its last checkpoint: the
//! header passes steps 2 to 4, every checkpoint verifies, and the chain
//! over the entries that are there meets each one.

use super::ckpt::{self, Mark};
use super::partial::Entries;
use super::walk::walk;
use super::{bound, header, Bound, RecordRefusal, Registry, Report, Verdict};
use crate::{Attester, Policy};
use alloc::vec::Vec;
use nsh_wire::{Reader, RecordHeader, WireError};

pub(super) fn cut_short<A: Attester + ?Sized>(
    record: &[u8],
    doc: &[u8],
    ckpts: &[&[u8]],
    expect: &Bound,
    policy: &Policy,
    attester: &A,
) -> Result<Report, RecordRefusal> {
    let mut r = Reader::new(record);
    let h = RecordHeader::read(&mut r).map_err(RecordRefusal::Parse)?;
    bound::check(&h, expect).map_err(RecordRefusal::Unbound)?;
    header::check(&h, doc, policy, attester)?;
    let mut marks = marks(ckpts, &h)?;
    let w = walk(h.bytes, Entries::new(r, h.start_ms), &mut marks).map_err(RecordRefusal::Ckpt)?;
    let seq = marks.iter().map(|m| m.seq).max();
    let seq = seq.ok_or(RecordRefusal::Parse(WireError::Short))?;
    let boxed = w.first_box.is_some_and(|b| b <= seq);
    Ok(report(&h, Verdict::CutShort { seq, boxed }))
}

pub(super) fn marks(ckpts: &[&[u8]], h: &RecordHeader<'_>) -> Result<Vec<Mark>, RecordRefusal> {
    ckpts.iter().map(|c| ckpt::check(c, h).map_err(RecordRefusal::Ckpt)).collect()
}

pub(super) fn report(h: &RecordHeader<'_>, verdict: Verdict) -> Report {
    let (session_id, subject, start_ms) = (*h.session_id, *h.subject, h.start_ms);
    Report { verdict, session_id, subject, start_ms, registry: Registry::RootThroughQuote }
}
