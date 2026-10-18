// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile programs: method bodies with bytes changed at random, loaded and
//! every method run. Each ends in a value or a fault, never a panic, and
//! leaves no mutex held and no frame behind.

use super::corpus::corpus;
use crate::enc::*;
use crate::exec::{ExecLimits, Machine};
use crate::load::Aml;
use crate::namespace::{Kind, ROOT};
use crate::run::Probe;

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn run_all(aml: &mut Aml, probe: &mut Probe) {
    let lim = ExecLimits { steps: 20_000, ..ExecLimits::DEFAULT };
    let methods: Vec<u32> = aml
        .ns
        .children(ROOT)
        .filter(|id| matches!(aml.ns.get(*id).unwrap().kind, Kind::Method(_)))
        .collect();
    for id in methods {
        let args = vec![crate::object::Object::Integer(1)];
        let _ = Machine::new(aml, probe, lim).evaluate(aml, id, args);
        assert!(aml.rt.mutexes.is_empty());
    }
}

#[test]
fn mutated_methods_end_in_a_value_or_a_fault() {
    let base = corpus();
    let mut seed = 0xA076_1D64_78BD_642Fu64;
    for _ in 0..3000 {
        let mut m = base.clone();
        for _ in 0..1 + next(&mut seed) % 3 {
            let at = (next(&mut seed) % m.len() as u64) as usize;
            m[at] = next(&mut seed) as u8;
        }
        let mut aml = Aml::default();
        let mut probe = Probe::default();
        let lim = ExecLimits { steps: 20_000, ..ExecLimits::DEFAULT };
        let loaded =
            Machine::new(&mut aml, &mut probe, lim).load_table(&mut aml, table(b"DSDT", 2, &m));
        if loaded.is_ok() {
            run_all(&mut aml, &mut probe);
        }
    }
}

#[test]
fn the_unmutated_corpus_runs_clean() {
    let mut aml = Aml::default();
    let mut probe = Probe::default();
    Machine::new(&mut aml, &mut probe, ExecLimits::DEFAULT)
        .load_table(&mut aml, table(b"DSDT", 2, &corpus()))
        .unwrap();
    for (path, args) in [("\\M1", 2), ("\\M2", 0), ("\\M3", 1), ("\\M4", 0), ("\\M5", 1)] {
        let id = crate::consts::at(&aml, path).unwrap();
        let a = (0..args).map(|_| crate::object::Object::Integer(1)).collect();
        let r = Machine::new(&mut aml, &mut probe, ExecLimits::DEFAULT).evaluate(&mut aml, id, a);
        assert!(r.is_ok(), "{path}: {r:?}");
    }
}
