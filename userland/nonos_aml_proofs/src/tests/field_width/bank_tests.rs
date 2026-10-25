// NONOS Operating System (AGPL-3.0-or-later)
//! The seeded units as BankFields against `model_ib`: the accesses in order,
//! the value read, and both banks afterwards. Seeds and floors are our
//! choice, from no source.

use super::gen::{below, case, Case};
use super::gen_ib::{bank_source, select_at};
use super::io::{arg, log, number};
use super::model_ib::bank_run;
use crate::exec::FaultKind;
use crate::probe_banks::Banked;
use crate::rig::Rig;
use std::collections::BTreeMap;

const SEEDS: u64 = 4000;

#[test]
fn bank_fields_select_their_bank_then_reach_only_it() {
    // Faults after the select, and units straddled.
    let mut tally = [0usize; 2];
    for seed in 0..SEEDS {
        let c = case(seed);
        let c = Case { space: if c.space == 3 { 1 } else { c.space }, ..c };
        let s = &mut (seed | 1);
        let bank = 1 + below(s, 2) as u8;
        let mut banks = BTreeMap::new();
        for b in 1..=2 {
            for a in c.base - 8..c.base + c.len + 8 {
                banks.insert((b, a), below(s, 256) as u8);
            }
        }
        let mut r = Rig::new(&bank_source(&c, bank)).connected();
        let (lo, hi) = (c.base, c.base + c.len + 8);
        let shown = Banked { space: c.space, lo, hi, select: select_at(&c), bytes: banks.clone() };
        r.probe.banks = Some(shown);
        let want = bank_run(&c, &mut banks, bank, None);
        let got = r.call("\\RD", vec![]);
        assert_eq!(log(&r, &c, 0), want.log, "seed {seed} read");
        match got {
            Ok(v) => assert!(!want.fault && number(&v) == want.value, "seed {seed}: {v:?}"),
            Err(f) => assert!(want.fault && f.kind == FaultKind::Index, "seed {seed}: {f:?}"),
        }
        let from = r.probe.log.len();
        let wrote = bank_run(&c, &mut banks, bank, Some(c.value));
        assert_eq!(r.call("\\WR", vec![arg(&c)]).is_err(), wrote.fault, "seed {seed}");
        assert_eq!(log(&r, &c, from), wrote.log, "seed {seed} write");
        assert_eq!(r.probe.banks.as_ref().unwrap().bytes, banks, "seed {seed} banks");
        tally[0] += usize::from(want.fault);
        tally[1] += usize::from(want.log.len() > 2);
    }
    assert!(tally[0] >= 600 && tally[1] >= 1950, "{tally:?}");
}
