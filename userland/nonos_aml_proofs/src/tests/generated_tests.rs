// NONOS Operating System (AGPL-3.0-or-later)
//! Generated programs: thousands of random integer expressions, encoded as
//! AML and run in the interpreter at both widths, against a model written
//! from the operators' definitions.

use super::model::gen;

#[path = "hostile_gen.rs"]
mod hostile_gen;
#[path = "hostile_table.rs"]
mod hostile_table;
#[path = "hostile_tests.rs"]
mod hostile_tests;

use crate::enc::*;
use crate::exec::FaultKind;
use crate::run::{boot, call, num};

#[test]
fn generated_expressions_match_the_model_at_both_widths() {
    let mut seed = 0x0123_4567_89AB_CDEFu64;
    for (rev, mask) in [(2u8, u64::MAX), (1, 0xFFFF_FFFF)] {
        for round in 0..1500 {
            let (expr, want) = gen(&mut seed, 5, mask);
            let (mut aml, _) = boot(&method("\\M", 0, &ret(&expr)), rev);
            let got = call(&mut aml, "\\M", vec![]);
            match want {
                Some(v) => assert_eq!(num(&got.expect("evaluates")), v, "rev {rev} round {round}"),
                None => assert_eq!(got.unwrap_err().kind, FaultKind::DivideByZero),
            }
        }
    }
}
