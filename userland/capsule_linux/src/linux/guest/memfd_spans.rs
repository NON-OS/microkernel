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

//! Which memfd each span of the guest maps, and from what file offset.
//! Pure, so the host proofs hold it.
//!
//! A memfd's bytes live only in the guest's mapping of it, so a wl_shm pool
//! reads its pixels there. The descriptor alone remembered the last address
//! it was mapped at, kept after the guest unmapped it: a pool made after the
//! client had let its mapping go (Qt does, as a resize replaces a buffer
//! before the bridge reads the request that made it) found an address with
//! nothing there and was refused, and its client killed for naming it. A
//! pool whose address the guest had mapped again for something else read
//! that instead, at its own stride. Each span here is cut as the guest
//! unmaps, so an address found is the memfd's own, now.

use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MemfdSpan {
    /// The memfd's identity, which its dups share (fd_make.rs).
    pub id: u32,
    /// Where the span starts in the guest, and how long it is.
    pub at: u64,
    pub len: u64,
    /// The file offset mapped at `at`.
    pub off: u64,
}

impl MemfdSpan {
    fn end(&self) -> u64 {
        self.at.saturating_add(self.len)
    }
}

/// `span` recorded as mapped now, over whatever it lands on. False, and
/// nothing recorded, for an empty span or one whose end or file end wraps.
pub fn record(spans: &mut Vec<MemfdSpan>, span: MemfdSpan) -> bool {
    if span.len == 0 || span.at.checked_add(span.len).is_none() {
        return false;
    }
    if span.off.checked_add(span.len).is_none() {
        return false;
    }
    let kept = cut(spans, span.at, span.len);
    *spans = kept;
    spans.push(span);
    true
}

/// `spans` with `[at, at + len)` taken out: a span split by it leaves its
/// two ends, each still naming the file bytes it maps.
pub fn cut(spans: &[MemfdSpan], at: u64, len: u64) -> Vec<MemfdSpan> {
    let end = at.saturating_add(len);
    let mut out = Vec::with_capacity(spans.len());
    for s in spans {
        if end <= s.at || at >= s.end() {
            out.push(*s);
            continue;
        }
        if at > s.at {
            out.push(MemfdSpan { len: at - s.at, ..*s });
        }
        if end < s.end() {
            let skip = end - s.at;
            out.push(MemfdSpan { at: end, len: s.end() - end, off: s.off + skip, ..*s });
        }
    }
    out
}

/// The guest address holding `len` bytes of memfd `id` from file offset
/// `off`, all in one span: the latest mapped, as a client draws into the
/// mapping it made last. None when no span holds them whole.
pub fn resolve(spans: &[MemfdSpan], id: u32, off: u64, len: u64) -> Option<u64> {
    let want_end = off.checked_add(len)?;
    spans.iter().rev().find_map(|s| {
        let file_end = s.off.checked_add(s.len)?;
        (s.id == id && s.off <= off && want_end <= file_end).then(|| s.at + (off - s.off))
    })
}

/// How far into memfd `id` its latest mapping reaches, as a file offset;
/// None when the guest has none of it mapped.
pub fn reach(spans: &[MemfdSpan], id: u32) -> Option<u64> {
    spans.iter().rev().find(|s| s.id == id).and_then(|s| s.off.checked_add(s.len))
}

/// mremap grew `[old, old + old_len)` in place by `grow`: the span ending
/// there is the same mapping, so it reaches further into its file. A span
/// whose end or file end would wrap keeps its length, as `record` refuses one.
pub fn grown(spans: &mut [MemfdSpan], old: u64, old_len: u64, grow: u64) {
    let Some(old_end) = old.checked_add(old_len) else { return };
    for s in spans.iter_mut() {
        if s.at <= old && s.end() == old_end {
            if let Some(len) = s.len.checked_add(grow).filter(|&l| holds(s.at, s.off, l)) {
                s.len = len;
            }
        }
    }
}

/// Whether a span at `at` from file offset `off` may be `len` long: neither
/// its end nor its file end wraps.
fn holds(at: u64, off: u64, len: u64) -> bool {
    at.checked_add(len).is_some() && off.checked_add(len).is_some()
}

/// mremap moved `[old, old + old_len)` to `to` and grew it to `new_len`:
/// the spans it held, at their new place, the last reaching to the end.
/// Called before the old span is cut. A span that would wrap at its new
/// place is not carried, so the memfd is found nowhere there.
pub fn moved(spans: &[MemfdSpan], old: u64, old_len: u64, to: u64, new_len: u64) -> Vec<MemfdSpan> {
    let Some(old_end) = old.checked_add(old_len) else { return Vec::new() };
    let mut out = Vec::new();
    for s in spans.iter().filter(|s| s.at < old_end && old < s.end()) {
        let (from, until) = (s.at.max(old), s.end().min(old_end));
        let mut len = until - from;
        if until == old_end && s.end() == old_end {
            len = len.saturating_add(new_len.saturating_sub(old_len));
        }
        let (Some(off), Some(at)) = (s.off.checked_add(from - s.at), to.checked_add(from - old))
        else {
            continue;
        };
        if holds(at, off, len) {
            out.push(MemfdSpan { id: s.id, at, len, off });
        }
    }
    out
}
