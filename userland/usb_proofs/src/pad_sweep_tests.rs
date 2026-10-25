// NONOS Operating System (AGPL-3.0-or-later)
//! Gamepad report descriptors over seeded runs, sound and then told one
//! lie, and reports read by what they lay out: pad_layout keeps the fields
//! pad_sweep states, and PadState tells what the rule in
//! pad_sweep/decode.rs tells, report by report.

use crate::descriptors::pad::{pad_layout, PadEvent, PadState};
use crate::pad_sweep::build::{descriptor, LIES};
use crate::pad_sweep::decode::{feed, Pad};
use crate::rng::Rng;

#[test]
fn a_gamepad_is_laid_out_and_read_as_its_descriptor_states() {
    let (mut told, mut pads) = (vec![0u32; LIES.len()], 0);
    for seed in 1..30_000u64 {
        let r = &mut Rng::seeded(seed.wrapping_mul(0x9E37_79B9));
        let lie = LIES[r.below(LIES.len())];
        let d = descriptor(r, lie);
        let at = format!("seed {seed}: {lie:?}");
        let got = pad_layout(&d.bytes);
        let seen = got.as_ref().map(|l| {
            let f = l
                .fields
                .iter()
                .map(|f| (f.page, f.usage, f.report_id, f.bit_offset, f.bit_size, f.min, f.max));
            (l.ids, f.collect::<Vec<_>>())
        });
        assert_eq!(seen, d.layout, "{at}: the layout");
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
        let (Some(layout), Some((ids, fields))) = (got, d.layout) else { continue };
        pads += 1;
        let (mut state, mut rule) = (PadState::new(layout), Pad::default());
        for k in 0..r.below(8) {
            let (id, bits) = d.reports[r.below(d.reports.len())];
            let id = if r.one_in(8) { id.wrapping_add(1) } else { id };
            let len = (bits as usize).div_ceil(8);
            let len = if r.one_in(4) { r.below(len + 1) } else { len };
            let mut raw = if ids { vec![id] } else { vec![] };
            raw.extend(r.bytes(len));
            if ids && r.one_in(12) {
                raw.clear();
            }
            let mut got = vec![];
            state.feed(&raw, |e| {
                got.push(match e {
                    PadEvent::Button(n, down) => (true, n, down as i32),
                    PadEvent::Axis(u, v) => (false, u, v),
                })
            });
            assert_eq!(got, feed(&mut rule, ids, &fields, &raw), "{at}, report {k}");
        }
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(*n >= 3_000, "{lie:?} was told only {n} times");
    }
    assert!(pads >= 2_000, "only {pads} gamepads");
}
