// NONOS Operating System (AGPL-3.0-or-later)
//! The protocol holds for every machine up to the bound, and each mutant
//! breaks it: a rule that no test can tell apart from its removal is not
//! being checked.

use crate::model::{Config, Flush};
use crate::search::explore;

/// Up to the initiator plus three APs, every configuration and mutant of the
/// round, with the boot CPU not asking any AP to park. One more AP multiplies
/// the states by about 30, so five CPUs are searched once, for the kernel's
/// rules, in `five_cpus_with_every_ap_coming_up`.
const CPUS: std::ops::RangeInclusive<usize> = 2..=4;

/// With the boot CPU free to ask any AP to park at any step of its bring-up,
/// up to the initiator plus two APs: the step counter multiplies the states
/// (395 thousand at three CPUs), and four do not fit in memory.
const PARK_CPUS: std::ops::RangeInclusive<usize> = 2..=3;

/// The kernel's rules without the park path, for the larger machines.
fn round(n: usize, online: usize, flush: Flush) -> Config {
    Config { may_ask_park: false, ..Config::kernel(n, online, flush) }
}

fn every_machine() -> impl Iterator<Item = Config> {
    CPUS.flat_map(|n| {
        (0..n).flat_map(move |online| {
            [Flush::Page, Flush::Full].into_iter().map(move |f| {
                if n <= *PARK_CPUS.end() {
                    Config::kernel(n, online, f)
                } else {
                    round(n, online, f)
                }
            })
        })
    })
}

fn holds(cfg: &Config) {
    let o = explore(cfg);
    if let Some((why, path)) = o.violation {
        panic!("{cfg:?}: {why}\n{path:#?}");
    }
    assert!(o.stuck.is_none(), "{cfg:?}: stuck at {:?}", o.stuck);
}

fn broken(cfg: &Config) -> &'static str {
    match explore(cfg).violation {
        Some((why, _)) => why,
        None => panic!("mutant survived: {cfg:?}"),
    }
}

const STALE: &str = "a cpu holds the old translation after the round returned";

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
fn mutant_target_online_only() {
    // The code before this rule: an AP that woke and cached the old page
    // before the write, and was read as `Starting`, is skipped and keeps it.
    for n in CPUS {
        let cfg = Config { target_starting: false, ..round(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), STALE);
    }
}

#[test]
fn mutant_no_ack_for_abandoned() {
    // A target given up on never answers; nobody answering for it is a round
    // that waits to its deadline and stops the machine.
    for n in CPUS {
        let cfg = Config { ack_for_abandoned: false, ..round(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), "an ack is owed by a cpu that will never answer");
    }
}

#[test]
fn mutant_select_before_write() {
    for n in CPUS {
        let cfg = Config { write_before_select: false, ..round(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), STALE);
    }
}

#[test]
fn mutant_ack_before_flush() {
    for n in CPUS {
        let cfg = Config { flush_before_ack: false, ..round(n, n - 1, Flush::Page) };
        assert_eq!(broken(&cfg), STALE);
    }
}

#[test]
fn global_pages_survive_a_cr3_reload_on_a_pge_cpu() {
    // With CR4.PGE set and GLOBAL kernel PTEs, a full flush by CR3 reload
    // keeps the stale entry; `invlpg` removes a global entry and is safe.
    for n in CPUS {
        let full = Config { initiator_pge: true, ..round(n, 0, Flush::Full) };
        assert_eq!(broken(&full), STALE);
        holds(&Config { initiator_pge: true, ..round(n, 0, Flush::Page) });
    }
}

#[test]
fn the_search_is_not_trivially_small() {
    // A model that collapsed to a few states would pass everything above for
    // no reason.
    let o = explore(&round(4, 0, Flush::Page));
    assert!(o.states > 50_000, "only {} states", o.states);
}

#[test]
fn a_parked_ap_leaves_the_set_only_after_init() {
    // Asked to park at any point of bring-up, an AP is a target until it has
    // parked and INIT has reset it; the round never returns while it may
    // still run with the old entry.
    for n in PARK_CPUS {
        holds(&Config::kernel(n, 0, Flush::Page));
    }
}

#[test]
fn mutant_drop_at_park_request() {
    // Dropped at `AbandonRequested`, before `Parked`: the AP is still in
    // bring-up with whatever it cached, and the round returns past it.
    for n in PARK_CPUS {
        let cfg = Config { drop_after_init: false, ..Config::kernel(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), STALE);
    }
}

#[test]
fn every_checkpoint_of_bring_up_is_outside_every_lock() {
    use crate::model::{Step, BRING_UP};
    let mut held = false;
    for step in BRING_UP {
        match step {
            Step::Lock => held = true,
            Step::Unlock => held = false,
            Step::Checkpoint => assert!(!held, "a checkpoint inside a lock"),
        }
    }
    assert!(!held, "bring_up ends holding a lock");
    for n in PARK_CPUS {
        holds(&Config::kernel(n, 0, Flush::Page));
    }
}

#[test]
fn mutant_checkpoint_inside_a_lock() {
    use crate::model::Step;
    const MOVED: &[Step] =
        &[Step::Checkpoint, Step::Lock, Step::Checkpoint, Step::Unlock, Step::Checkpoint];
    for n in PARK_CPUS {
        let cfg = Config { bring_up: MOVED, ..Config::kernel(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), "an AP can park while holding a lock");
    }
}
