// NONOS Operating System (AGPL-3.0-or-later)
//! Every seeded case read, then written, against the byte model. Seeds and
//! floors are our choice, from no source.

use super::gen::{case, source};
use super::io::{arg, log, number};
use super::model::run;
use super::width::width;
use crate::exec::FaultKind;
use crate::rig::Rig;
use std::collections::BTreeMap;

const SEEDS: u64 = 6000;

#[test]
fn every_unit_reads_and_writes_as_the_byte_model_says() {
    // Straddling reads, faults, partial writes per rule, Preserve reads,
    // buffer-sized units, AnyAcc wider than a byte.
    // And AnyAcc kept narrow by the region's end.
    let mut tally = [0usize; 9];
    for seed in 0..SEEDS {
        let c = case(seed);
        let mut r = Rig::new(&source(&c)).connected();
        let mut mem = BTreeMap::new();
        for (i, b) in c.fill.iter().enumerate() {
            let a = c.base - 8 + i as u64;
            r.probe.bytes.insert((c.space, a), *b);
            mem.insert(a, *b);
        }
        let want = run(&c, &mut mem, None);
        let got = r.call("\\RD", vec![]);
        assert_eq!(log(&r, &c, 0), want.log, "seed {seed} read");
        match got {
            Ok(v) => assert!(!want.fault && number(&v) == want.value, "seed {seed}: {v:?}"),
            Err(f) => assert!(want.fault && f.kind == FaultKind::Index, "seed {seed}: {f:?}"),
        }
        let from = r.probe.log.len();
        let wrote = run(&c, &mut mem, Some(c.value));
        let got = r.call("\\WR", vec![arg(&c)]);
        assert_eq!(log(&r, &c, from), wrote.log, "seed {seed} write");
        assert_eq!(got.is_err(), wrote.fault, "seed {seed} write");
        for (a, b) in &mem {
            assert_eq!(r.probe.bytes.get(&(c.space, *a)), Some(b), "seed {seed} byte {a:#x}");
        }
        let w = width(&c);
        tally[0] += usize::from(want.log.len() > 1);
        tally[1] += usize::from(want.fault);
        tally[2 + c.rule as usize] +=
            usize::from(wrote.log.iter().any(|l| l.1 as u64 * 8 > u64::from(c.bits)));
        tally[5] +=
            usize::from(c.rule == 0 && wrote.log.iter().filter(|l| l.2.is_none()).count() > 0);
        tally[6] += usize::from(c.bits > 64 && !want.fault);
        tally[7] += usize::from(matches!(c.access, 0 | 5) && w > 1 && c.space != 3);
        let fits = |w: u64| u64::from(c.off) / (8 * w) == u64::from(c.off + c.bits - 1) / (8 * w);
        let wide = [1, 2, 4, 8].into_iter().find(|w| fits(*w)).unwrap_or(1);
        tally[8] += usize::from(matches!(c.access, 0 | 5) && c.space != 3 && wide > w);
    }
    let floors = [3200, 750, 500, 500, 450, 1550, 350, 420, 160];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
