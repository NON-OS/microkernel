// NONOS Operating System (AGPL-3.0-or-later)
//! Every seeded case against the model, with floors on the cases that
//! matter: a load that announces, a _REG in a later table than its device,
//! a connect made twice, and each load path. Seeds and floors are our
//! choice, from no source.

use super::fixture::tables;
use super::gen::{case, Step};
use super::model::expect;
use crate::exec::{ExecLimits, Machine};
use crate::object::Object;
use crate::probe::Probe;
use crate::rig::Rig;

const SEEDS: u64 = 3000;

#[test]
fn every_case_calls_reg_as_the_model_says() {
    let mut tally = [0usize; 6];
    for seed in 0..SEEDS {
        let c = case(seed);
        let (dsdt, later) = tables(&c);
        let mut r = Rig::with(&dsdt, Probe { tables: later.clone(), ..Default::default() });
        let mut seen = Vec::new();
        for step in &c.steps {
            let before = r.probe.debug.len();
            match *step {
                Step::Connect(s) => {
                    assert!(r.connect(s).is_empty(), "seed {seed}");
                    tally[2] += usize::from(seen.contains(&s));
                    seen.push(s);
                }
                Step::Load(k, path) => {
                    match path {
                        0 => {
                            let mut m = Machine::new(&mut r.aml, &mut r.probe, ExecLimits::DEFAULT);
                            m.load_table(&mut r.aml, later[k - 1].clone()).expect("loads");
                        }
                        p => {
                            let m = if p == 1 { format!("\\LT{k}") } else { format!("\\LB{k}") };
                            let got = r.call(&m, vec![]).expect("loads");
                            assert!(matches!(got, Object::Table(_)), "seed {seed}: {got:?}");
                        }
                    }
                    tally[3 + path as usize] += 1;
                    tally[0] += usize::from(r.probe.debug.len() > before);
                }
            }
        }
        tally[1] += usize::from(c.devs.iter().any(|d| d.reg_in.is_some_and(|t| t > d.table)));
        assert_eq!(r.probe.debug, expect(&c), "seed {seed}");
    }
    let floors = [600, 900, 2000, 800, 800, 800];
    assert!(tally.iter().zip(floors).all(|(t, f)| *t >= f), "{tally:?}");
}
