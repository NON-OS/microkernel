// NONOS Operating System (AGPL-3.0-or-later)
//! The decision held to its rules over every input in `cases::every`.

use crate::cases::every;
use crate::spectre::answer::Answer;
use crate::spectre::decide::decide;
use crate::spectre::plan::{Bhb, Ssbd, Why, V2};

/// An ID register that says "not affected" or "implemented" decides,
/// whatever the firmware answered.
#[test]
fn id_registers_outrank_firmware() {
    every(|ids, fw| {
        let plan = decide(ids, fw);
        if ids.csv2() >= 1 {
            assert_eq!(plan.v2, V2::NotAffected(Why::Id));
        }
        if ids.ecbhb() >= 1 || ids.csv2() == 3 {
            assert_eq!(plan.bhb, Bhb::NotAffected(Why::Id));
        } else if ids.clrbhb() >= 1 {
            assert_eq!(plan.bhb, Bhb::Clrbhb);
        }
        if ids.ssbs() >= 1 {
            assert_eq!(plan.ssbd, Ssbd::Ssbs);
        }
    });
}

/// Where the ID registers are silent, the firmware's answer is taken.
#[test]
fn firmware_answers_followed_where_ids_are_silent() {
    every(|ids, fw| {
        let plan = decide(ids, fw);
        let expect = |a: Answer| match a {
            Answer::Call => Some(true),
            Answer::NotRequired => Some(false),
            Answer::Unknown => None,
        };
        if ids.csv2() == 0 {
            match expect(fw.wa1) {
                Some(true) => assert_eq!(plan.v2, V2::Firmware),
                Some(false) => assert_eq!(plan.v2, V2::NotAffected(Why::Firmware)),
                None => assert!(matches!(plan.v2, V2::NotAffected(Why::Core) | V2::Vulnerable)),
            }
        }
        if ids.ecbhb() == 0 && ids.csv2() < 3 && ids.clrbhb() == 0 {
            match expect(fw.wa3) {
                Some(true) => assert_eq!(plan.bhb, Bhb::Firmware),
                Some(false) => assert_eq!(plan.bhb, Bhb::NotAffected(Why::Firmware)),
                None => assert!(!matches!(plan.bhb, Bhb::Firmware | Bhb::Clrbhb)),
            }
        }
        if ids.ssbs() == 0 {
            match expect(fw.wa2) {
                Some(true) => assert_eq!(plan.ssbd, Ssbd::Firmware),
                Some(false) => assert_eq!(plan.ssbd, Ssbd::NotAffected(Why::Firmware)),
                None => {
                    assert!(matches!(plan.ssbd, Ssbd::NotAffected(Why::Core) | Ssbd::Vulnerable))
                }
            }
        }
    });
}
