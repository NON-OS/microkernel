// NONOS Operating System (AGPL-3.0-or-later)
//! More rows by name: cores that answer for themselves, and the decoding of
//! what firmware says.

use crate::cases::{ids, A53, A76, ARM};
use crate::spectre::answer::{Answer, Firmware};
use crate::spectre::decide::decide;
use crate::spectre::plan::{Bhb, Meltdown, Ssbd, Why, V2};

const SILENT: Firmware =
    Firmware { wa1: Answer::Unknown, wa2: Answer::Unknown, wa3: Answer::Unknown };

/// Cortex-A76 reports CSV2 and SSBS, and has the 24-branch loop.
#[test]
fn a76_from_its_id_registers() {
    let plan = decide(&ids([1, 1, 1, 0, 0, 0], (ARM, A76)), &SILENT);
    assert_eq!(plan.v2, V2::NotAffected(Why::Id));
    assert_eq!((plan.bhb, plan.ssbd), (Bhb::Loop(24), Ssbd::Ssbs));
    assert_eq!(plan.meltdown, Meltdown::NotAffected(Why::Id));
}

/// Cortex-A53 is in order: unaffected by every part, by the core list.
#[test]
fn a53_is_quiet() {
    let core = Why::Core;
    let plan = decide(&ids([0; 6], (ARM, A53)), &SILENT);
    assert_eq!((plan.v2, plan.bhb), (V2::NotAffected(core), Bhb::NotAffected(core)));
    assert_eq!((plan.ssbd, plan.meltdown), (Ssbd::NotAffected(core), Meltdown::NotAffected(core)));
}

/// Another implementer with silent ID registers is neither trusted nor
/// guessed at: open where nothing answers, Meltdown unknown.
#[test]
fn another_implementer_is_not_guessed() {
    let plan = decide(&ids([0; 6], (0x61, 0x022)), &SILENT);
    assert_eq!((plan.v2, plan.bhb, plan.ssbd), (V2::Vulnerable, Bhb::Vulnerable, Ssbd::Vulnerable));
    assert_eq!(plan.meltdown, Meltdown::Unknown);
}

#[test]
fn smccc_answers_decode() {
    let rows =
        [(Some(0), Answer::Call), (Some(1), Answer::NotRequired), (Some(-2), Answer::NotRequired)];
    for (ret, answer) in rows {
        assert_eq!(Answer::from_ret(ret), answer);
    }
    for ret in [None, Some(-1), Some(-3), Some(2), Some(i64::MIN)] {
        assert_eq!(Answer::from_ret(ret), Answer::Unknown);
    }
}
