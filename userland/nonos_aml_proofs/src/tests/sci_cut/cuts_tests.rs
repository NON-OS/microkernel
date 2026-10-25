// NONOS Operating System (AGPL-3.0-or-later)
//! `CUTS` runs in a row cut by the budget leave a GPE off; fewer, with a
//! finished run between, leave it armed.

use super::fixture::scene;
use super::run::{sci, start};
use crate::events::CUTS;
use crate::probe::Probe;
use crate::tests::sci_rig::bits;

#[test]
fn cuts_in_a_row_leave_it_off_and_a_finished_run_starts_the_count_again() {
    for seed in 0..20 {
        let sc = scene(seed);
        // A run that finishes starts the count again.
        let (mut rig, mut aml, g) = start(&sc);
        for steps in [1, 1_000_000, 1] {
            let runs = if steps == 1 { CUTS - 1 } else { 1 };
            for _ in 0..runs {
                let (report, ..) = sci(&mut rig, &mut aml, &mut Probe::default(), g, steps);
                assert!(report.disabled.is_empty(), "seed {seed}: never CUTS in a row");
            }
            aml.rt.notifications.clear();
        }
        let (mut rig, mut aml, g) = start(&sc);
        for n in 1..=CUTS {
            let (report, ..) = sci(&mut rig, &mut aml, &mut Probe::default(), g, 1);
            assert_eq!(report.disabled.is_empty(), n < CUTS, "seed {seed}: cut {n}");
        }
        assert_eq!(bits(&rig.regs, g), (false, false), "seed {seed}: left off after {CUTS}");
    }
}
