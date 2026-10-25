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

//! What the plan tests share: a span, a run, and the byte-by-byte check.

use super::plan::{plan, Span, MAX_SPANS};

pub(super) fn span(start: u64, end: u64) -> Span {
    Span { start, end }
}

pub(super) fn run(wipe: Span, keep: &[Span]) -> Vec<Span> {
    let mut out = [span(0, 0); MAX_SPANS];
    let n = plan(wipe, keep, &mut out).expect("within MAX_KEEP");
    out[..n].to_vec()
}

pub(super) fn covered(spans: &[Span], at: u64) -> bool {
    spans.iter().any(|s| s.start <= at && at < s.end)
}

/// Every byte the plan wipes is in the heap and in no kept span, and every
/// heap byte outside the kept spans is wiped: checked byte by byte.
pub(super) fn exact(wipe: Span, keep: &[Span]) {
    let spans = run(wipe, keep);
    for w in spans.windows(2) {
        assert!(w[0].end <= w[1].start, "in order and apart: {spans:?}");
    }
    let lo = wipe.start.saturating_sub(64);
    for at in lo..wipe.end + 64 {
        let in_heap = wipe.start <= at && at < wipe.end;
        let kept = keep.iter().any(|k| k.start <= at && at < k.end);
        assert_eq!(
            covered(&spans, at),
            in_heap && !kept,
            "byte {at:#x}: heap {wipe:?} keep {keep:?} plan {spans:?}"
        );
    }
}
