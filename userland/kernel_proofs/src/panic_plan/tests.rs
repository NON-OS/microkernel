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

use super::panic_plan::{plan, Span, MAX_KEEP, MAX_SPANS};

fn span(start: u64, end: u64) -> Span {
    Span { start, end }
}

fn run(wipe: Span, keep: &[Span]) -> Vec<Span> {
    let mut out = [span(0, 0); MAX_SPANS];
    let n = plan(wipe, keep, &mut out).expect("within MAX_KEEP");
    out[..n].to_vec()
}

fn covered(spans: &[Span], at: u64) -> bool {
    spans.iter().any(|s| s.start <= at && at < s.end)
}

/// Every byte the plan wipes is in the heap and in no kept span, and every
/// heap byte outside the kept spans is wiped: checked byte by byte.
fn exact(wipe: Span, keep: &[Span]) {
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

#[test]
fn a_heap_that_overlaps_nothing_is_wiped_whole() {
    let heap = span(0x1000, 0x9000);
    assert_eq!(run(heap, &[span(0x10_0000, 0x11_0000), span(0x0, 0x800)]), vec![heap]);
}

#[test]
fn the_running_stack_is_never_wiped_wherever_it_sits() {
    let heap = span(0x1000, 0x5000);
    for (a, b) in
        [(0x0, 0x1800), (0x2000, 0x3000), (0x4800, 0x6000), (0x0, 0x9000), (0x1000, 0x5000)]
    {
        exact(heap, &[span(a, b)]);
    }
}

#[test]
fn the_vault_inside_the_heap_is_wiped_around_every_kept_span() {
    // The key vault's map lives in the heap; an address standing for it, at
    // either end, in the middle, and between kept spans, is always covered.
    let heap = span(0x1000, 0x9000);
    let keep = [span(0x2000, 0x2400), span(0x5000, 0x6000), span(0x8f00, 0x9100)];
    let spans = run(heap, &keep);
    for vault in [0x1000, 0x1fff, 0x2400, 0x4000, 0x6000, 0x8eff] {
        assert!(covered(&spans, vault), "vault at {vault:#x}: {spans:?}");
    }
    exact(heap, &keep);
}

#[test]
fn many_layouts_are_exact() {
    let mut x = 0x2545_f491_4f6c_dd1du64;
    let mut next = |m: u64| {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x % m
    };
    for _ in 0..400 {
        let a = next(0x800);
        let heap = span(a, a + next(0x800));
        let keep: Vec<Span> = (0..next(MAX_KEEP as u64 + 1))
            .map(|_| {
                let s = next(0x1000);
                span(s, s + next(0x300))
            })
            .collect();
        exact(heap, &keep);
    }
}

#[test]
fn more_kept_spans_than_it_can_hold_wipe_nothing() {
    let keep = vec![span(0, 1); MAX_KEEP + 1];
    let mut out = [span(0, 0); MAX_SPANS];
    assert_eq!(plan(span(0x1000, 0x2000), &keep, &mut out), None);
}

#[test]
fn an_empty_heap_plans_nothing() {
    assert!(run(span(0x2000, 0x2000), &[span(0, 0x10)]).is_empty());
    assert!(run(span(0x3000, 0x2000), &[]).is_empty());
}
