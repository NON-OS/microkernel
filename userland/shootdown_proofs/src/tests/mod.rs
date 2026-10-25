// NONOS Operating System (AGPL-3.0-or-later)
//! The protocol holds for every machine up to the bound, and each mutant
//! breaks it: a rule that no test can tell apart from its removal is not
//! being checked.

use crate::model::{Config, Flush, Pge};
use crate::search::explore;

/// Up to the initiator plus three APs, every configuration and mutant of the
/// round, with the boot CPU not asking any AP to park. One more AP multiplies
/// the states by about 30, so five CPUs are searched once, for the kernel's
/// rules, in `five_cpus_with_every_ap_coming_up`.
pub(super) const CPUS: std::ops::RangeInclusive<usize> = 2..=4;

/// With the boot CPU free to ask any AP to park at any step of its bring-up,
/// up to the initiator plus two APs: the step counter multiplies the states
/// (395 thousand at three CPUs), and four do not fit in memory.
pub(super) const PARK_CPUS: std::ops::RangeInclusive<usize> = 2..=3;

/// The kernel's rules without the park path, for the larger machines.
pub(super) fn round(n: usize, online: usize, flush: Flush) -> Config {
    Config { may_ask_park: false, ..Config::kernel(n, online, flush) }
}

pub(super) fn every_machine() -> impl Iterator<Item = Config> {
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

pub(super) fn holds(cfg: &Config) {
    let o = explore(cfg);
    if let Some((why, path)) = o.violation {
        panic!("{cfg:?}: {why}\n{path:#?}");
    }
    assert!(o.stuck.is_none(), "{cfg:?}: stuck at {:?}", o.stuck);
}

pub(super) fn broken(cfg: &Config) -> &'static str {
    match explore(cfg).violation {
        Some((why, _)) => why,
        None => panic!("mutant survived: {cfg:?}"),
    }
}

pub(super) const STALE: &str = "a cpu holds the old translation after the round returned";

mod mutant_tests;
mod park_tests;
mod round_tests;
