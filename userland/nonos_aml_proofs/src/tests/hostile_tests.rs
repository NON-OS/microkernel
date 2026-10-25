// NONOS Operating System (AGPL-3.0-or-later)
//! F-97: the interpreter against seeded hostile tables. Every method of
//! every table runs under small limits: none panics, none takes a step or a
//! byte past its budget, and each shape that must fault faults as it must.

use super::hostile_gen::{nested_pkg, soup, Want};
use super::hostile_table::{hostile, LIMITS};
use crate::enc::*;
use crate::exec::{FaultKind, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::run::Probe;

#[test]
fn hostile_tables_never_panic_or_pass_a_bound() {
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    // Faults by kind (steps, depth, bytes) and values: none may stay unseen.
    let mut seen = [0u32; 4];
    for round in 0..300 {
        let (body, wants) = hostile(&mut s, round % 1000);
        let (mut aml, mut probe) = (Aml::default(), Probe::default());
        let t = table(b"DSDT", 2, &body);
        if Machine::new(&mut aml, &mut probe, LIMITS).load_table(&mut aml, t).is_err() {
            continue;
        }
        for (seg, arg0, want) in wants {
            let Some(id) = crate::consts::at(&aml, &format!("\\{seg}")) else { continue };
            let mut m = Machine::new(&mut aml, &mut probe, LIMITS);
            let got = m.evaluate(&mut aml, id, vec![Object::Integer(arg0)]);
            assert!(m.steps <= LIMITS.steps + 1, "round {round} {seg}: {} steps", m.steps);
            assert!(m.alloc <= LIMITS.alloc, "round {round} {seg}: {} bytes", m.alloc);
            assert!(m.frames.is_empty() && aml.rt.mutexes.is_empty(), "round {round} {seg}");
            match (want, got) {
                (Want::Any, _) => {}
                (Want::Fault(k), Err(f)) => {
                    assert_eq!(f.kind, k, "round {round} {seg}");
                    let slot = [FaultKind::Steps, FaultKind::CallDepth, FaultKind::Alloc];
                    seen[slot.iter().position(|x| *x == k).unwrap_or(0)] += 1;
                }
                (Want::Value(v), Ok(Object::Integer(n))) => {
                    assert_eq!(n, v, "round {round} {seg}");
                    seen[3] += 1;
                }
                (_, other) => panic!("round {round} {seg}: {other:?}"),
            }
        }
    }
    assert!(seen.iter().all(|n| *n >= 50), "every bound met often: {seen:?}");
}

#[test]
fn packages_nested_past_the_parser_depth_are_refused_whole() {
    let mut s = 0x2545_F491_4F6C_DD1Du64;
    for depth in [63u32, 64, 65, 100, 400] {
        let body = name_obj("DEEP", &nested_pkg(&mut s, depth));
        let (mut aml, mut probe) = (Aml::default(), Probe::default());
        let t = table(b"DSDT", 2, &body);
        let r = Machine::new(&mut aml, &mut probe, LIMITS).load_table(&mut aml, t);
        if depth >= 64 {
            assert!(r.is_err(), "depth {depth} loads");
        }
    }
    for round in 0..2000 {
        let body = soup(&mut s, 512);
        let (mut aml, mut probe) = (Aml::default(), Probe::default());
        let t = table(b"DSDT", 2, &body);
        let _ = Machine::new(&mut aml, &mut probe, LIMITS).load_table(&mut aml, t);
        let _ = round;
    }
}
