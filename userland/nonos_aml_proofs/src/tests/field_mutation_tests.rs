// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile field programs: regions, fields, index fields, buffer fields and
//! Global Lock fields with bytes changed at random, every space connected,
//! every method run. Each ends in a value or a fault, never a panic, with
//! the Global Lock given back.

use super::field_corpus::corpus;
use crate::enc::*;
use crate::exec::{ExecLimits, Machine};
use crate::load::Aml;
use crate::namespace::{Kind, ROOT};
use crate::object::Object;
use crate::probe::Probe;
use crate::region::Space;

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn mutated_field_programs_end_in_a_value_or_a_fault() {
    let base = corpus();
    let mut seed = 0x5851_F42D_4C95_7F2Du64;
    let lim = ExecLimits { steps: 20_000, ..ExecLimits::DEFAULT };
    for _ in 0..3000 {
        let mut m = base.clone();
        for _ in 0..1 + next(&mut seed) % 3 {
            let at = (next(&mut seed) % m.len() as u64) as usize;
            m[at] = next(&mut seed) as u8;
        }
        let mut aml = Aml::default();
        let mut probe = Probe { facs: Some(0), ..Default::default() };
        if Machine::new(&mut aml, &mut probe, lim)
            .load_table(&mut aml, table(b"DSDT", 2, &m))
            .is_err()
        {
            continue;
        }
        for s in 0..=0x0A {
            Machine::new(&mut aml, &mut probe, lim).connect(&mut aml, Space(s));
        }
        let methods: Vec<u32> = aml
            .ns
            .children(ROOT)
            .filter(|id| matches!(aml.ns.get(*id).unwrap().kind, Kind::Method(_)))
            .collect();
        for id in methods {
            let args = vec![Object::buffer(vec![1; 8])];
            let _ = Machine::new(&mut aml, &mut probe, lim).evaluate(&mut aml, id, args);
            assert_eq!(aml.rt.global_lock, 0, "the Global Lock is given back");
        }
    }
}
