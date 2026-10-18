// NONOS Operating System (AGPL-3.0-or-later)
//! The entry word: what the EL0 vectors run for each plan, and the merge of
//! many CPUs' words, which must never drop what any one CPU needs.

use crate::cases::every;
use crate::spectre::decide::decide;
use crate::spectre::entry_word::{
    merge, word, CALL_MASK, CALL_WA1, CALL_WA3, CLRBHB, HVC, LOOP_MASK, LOOP_SHIFT,
};
use crate::spectre::plan::{Bhb, Plan, V2};

/// What `SPECTRE_FROM_EL0` reads out of a word, as vectors.S decodes it.
fn runs(w: u32) -> (u32, bool, u32, bool) {
    (w & CALL_MASK, w & CLRBHB != 0, (w & LOOP_MASK) >> LOOP_SHIFT, w & HVC != 0)
}

fn needs(plan: &Plan) -> (u32, bool, u32) {
    let call = match (plan.bhb, plan.v2) {
        (Bhb::Firmware, _) => CALL_WA3,
        (_, V2::Firmware) => CALL_WA1,
        _ => 0,
    };
    let k = if let Bhb::Loop(k) = plan.bhb { u32::from(k) } else { 0 };
    (call, plan.bhb == Bhb::Clrbhb, k)
}

#[test]
fn each_plan_is_run_as_decided() {
    every(|ids, fw| {
        let plan = decide(ids, fw);
        for hvc in [false, true] {
            let (call, clr, k, via_hvc) = runs(word(&plan, hvc));
            assert_eq!((call, clr, k), needs(&plan));
            assert_eq!(via_hvc, hvc && call != 0);
        }
    });
}

/// Merging is commutative, associative, idempotent, and gives exactly the
/// stronger call, the longer loop, and CLRBHB if either did, so it never
/// loses a need. ARCH_WORKAROUND_3 covers _1.
#[test]
fn merging_keeps_every_cpus_needs() {
    let mut words = Vec::new();
    every(|ids, fw| {
        let w = word(&decide(ids, fw), false);
        if !words.contains(&w) {
            words.push(w);
        }
    });
    assert!(words.len() >= 6, "the walk reaches every kind of word");
    for &a in &words {
        assert_eq!(merge(a, a), a);
        assert_eq!(merge(0, a), a);
        for &b in &words {
            let m = merge(a, b);
            assert_eq!(m, merge(b, a));
            let (x, y, all) = (runs(a), runs(b), runs(m));
            // No more than the stronger of the two either: a longer loop
            // or a call no CPU asked for is time spent on every entry.
            assert_eq!((all.0, all.1, all.2), (x.0.max(y.0), x.1 || y.1, x.2.max(y.2)));
            for &c in &words {
                assert_eq!(merge(merge(a, b), c), merge(a, merge(b, c)));
            }
        }
    }
}
