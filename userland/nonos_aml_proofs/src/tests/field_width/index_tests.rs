// NONOS Operating System (AGPL-3.0-or-later)
//! The seeded units as IndexFields against `model_ib`: the accesses in order,
//! the value read, and the hidden bytes afterwards. Seeds and floors are our
//! choice, from no source.

use super::gen::{below, case, Case};
use super::gen_ib::index_source;
use super::io::{arg, log, number};
use super::model_ib::{index_run, index_width};
use crate::probe_banks::Window;
use crate::rig::Rig;
use std::collections::BTreeMap;

const SEEDS: u64 = 4000;

#[test]
fn index_fields_write_the_index_before_each_unit() {
    // Units straddled, Preserve writes that went round twice.
    let mut tally = [0usize; 2];
    for seed in 0..SEEDS {
        let c = case(seed);
        let s = &mut (seed | 1);
        let (il, w) = ([1, 2, 4][below(s, 3) as usize], index_width(&c));
        let mut r = Rig::new(&index_source(&c, il, w)).connected();
        let mut hidden: BTreeMap<u64, u8> = (0..40).map(|i| (i, below(s, 256) as u8)).collect();
        let bytes = hidden.clone();
        r.probe.window = Some(Window { space: 1, index: 0x70, index_len: il, data: 0x78, bytes });
        let c1 = Case { space: 1, ..c };
        let want = index_run(&c1, &mut hidden, il, None);
        let got = r.call("\\RD", vec![]).expect("an index field reads");
        assert_eq!((log(&r, &c1, 0), number(&got)), (want.log.clone(), want.value), "seed {seed}");
        let from = r.probe.log.len();
        let wrote = index_run(&c1, &mut hidden, il, Some(c1.value));
        r.call("\\WR", vec![arg(&c1)]).expect("an index field writes");
        assert_eq!(log(&r, &c1, from), wrote.log, "seed {seed} write");
        assert_eq!(r.probe.window.as_ref().unwrap().bytes, hidden, "seed {seed} bytes");
        tally[0] += usize::from(want.log.len() > 2);
        tally[1] += usize::from(wrote.log.len() > 2 * (want.log.len() / 2));
    }
    assert!(tally[0] >= 2100 && tally[1] >= 1050, "{tally:?}");
}
