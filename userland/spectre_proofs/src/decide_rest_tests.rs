// NONOS Operating System (AGPL-3.0-or-later)
//! What the decision may only say with cause: "vulnerable" only where no
//! means exists, a mitigation only where its means does.

use crate::cases::every;
use crate::spectre::answer::Answer;
use crate::spectre::cores::{bhb_loop, in_order};
use crate::spectre::decide::decide;
use crate::spectre::plan::{Bhb, Ssbd, Why, V2};

#[test]
fn vulnerable_only_where_nothing_could_be_done() {
    every(|ids, fw| {
        let plan = decide(ids, fw);
        let core = (ids.implementer(), ids.part());
        let quiet = in_order(core.0, core.1);
        if plan.v2 == V2::Vulnerable {
            assert!(ids.csv2() == 0 && fw.wa1 == Answer::Unknown && !quiet);
        }
        if plan.bhb == Bhb::Vulnerable {
            assert!(ids.ecbhb() == 0 && ids.csv2() < 3 && ids.clrbhb() == 0);
            assert!(fw.wa3 == Answer::Unknown && bhb_loop(core.0, core.1).is_none() && !quiet);
        }
        if plan.ssbd == Ssbd::Vulnerable {
            assert!(ids.ssbs() == 0 && fw.wa2 == Answer::Unknown && !quiet);
        }
    });
}

#[test]
fn the_loop_only_with_a_published_count_and_nothing_better() {
    every(|ids, fw| {
        if let Bhb::Loop(k) = decide(ids, fw).bhb {
            assert_eq!(bhb_loop(ids.implementer(), ids.part()), Some(k));
            assert!([8, 24, 32].contains(&k));
            assert!(ids.clrbhb() == 0 && fw.wa3 == Answer::Unknown);
        }
    });
}

#[test]
fn a_core_list_is_the_last_word_never_the_first() {
    every(|ids, fw| {
        let plan = decide(ids, fw);
        if plan.v2 == V2::NotAffected(Why::Core) {
            assert!(ids.csv2() == 0 && fw.wa1 == Answer::Unknown);
        }
        if plan.ssbd == Ssbd::NotAffected(Why::Core) {
            assert!(ids.ssbs() == 0 && fw.wa2 == Answer::Unknown);
        }
        if plan.bhb == Bhb::NotAffected(Why::Core) {
            assert!(fw.wa3 == Answer::Unknown && ids.clrbhb() == 0);
        }
    });
}
