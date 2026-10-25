// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel's rules hold on every machine up to the bound.

use super::*;

#[test]
fn the_round_leaves_no_stale_entry_on_any_machine() {
    for cfg in every_machine() {
        holds(&cfg);
    }
}

#[test]
fn an_ap_that_comes_up_during_the_round_is_covered() {
    // Every AP offline at the start: each may be started, wake, cache the old
    // page, claim and publish, or be given up on, at any point of the round.
    for n in CPUS {
        holds(&round(n, 0, Flush::Page));
    }
}

#[test]
fn five_cpus_with_every_ap_coming_up() {
    holds(&round(5, 0, Flush::Page));
}

#[test]
fn the_search_is_not_trivially_small() {
    // A model that collapsed to a few states would pass everything above for
    // no reason.
    let o = explore(&round(4, 0, Flush::Page));
    assert!(o.states > 50_000, "only {} states", o.states);
}
