// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What setup's Qwen step offers and starts on, over the real pins in the
//! order setup's build.rs writes them (summed per tier, smallest first) and
//! the fetcher's own fit rule (need.rs). Only tiers that fit are offered,
//! smallest first. For an install, a tier fits when its run leaves 1 GiB;
//! on an amnesic stick, whose volume the kernel holds in memory, when its
//! file and its run fit in memory less the larger of 1 GiB and a quarter.
//! The step starts on the stick tier, Qwen3 0.6B, whenever it fits.

use std::collections::BTreeMap;

use crate::need::{memory, session_keep, tier_fits, Room, STICK_TIER};
use crate::qwen_default::{default_row, offered, For};
use crate::qwen_pins::all;

mod defaults;
mod proven;

const GIB: u64 = 1 << 30;

/* The tiers as setup lists them: each one's files summed, smallest first. */
fn tiers() -> Vec<(&'static [u8], u64)> {
    let mut sum: BTreeMap<&'static str, u64> = BTreeMap::new();
    for p in all() {
        *sum.entry(p.tier).or_default() += p.bytes;
    }
    let mut out: Vec<(&'static [u8], u64)> =
        sum.into_iter().map(|(t, b)| (t.as_bytes(), b)).collect();
    out.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(b.0)));
    out
}

fn names(t: &[(&'static [u8], u64)], offered: &[u8]) -> Vec<&'static str> {
    offered.iter().map(|&i| core::str::from_utf8(t[i as usize].0).unwrap()).collect()
}

fn chosen(
    t: &[(&'static [u8], u64)],
    offered: &[u8],
    total: u64,
    who: For,
) -> Option<&'static str> {
    let row = default_row(t, offered, Some(total), who) as usize;
    let i = *offered.get(row.checked_sub(1)?)?;
    Some(core::str::from_utf8(t[i as usize].0).unwrap())
}

#[test]
fn the_smallest_tiers_lead_the_list() {
    let t = tiers();
    assert_eq!(t.len(), 19);
    assert_eq!(core::str::from_utf8(t[0].0).unwrap(), "small");
    assert_eq!(core::str::from_utf8(t[1].0).unwrap(), "qwen3-0.6b");
}

/* Every tier offered is one need.rs fits, in the list's order, and every tier it fits is offered. */
#[test]
fn only_what_fits_is_offered_smallest_first() {
    let t = tiers();
    for total in [GIB, 2 * GIB, 3 * GIB, 4 * GIB, 8 * GIB, 16 * GIB, 32 * GIB, 128 * GIB] {
        for (who, room) in [(For::Installed, Room::Disk), (For::Session, Room::Memory)] {
            let o = offered(&t, Some(total), who);
            assert!(o.windows(2).all(|w| w[0] < w[1]), "{total} {who:?}: not smallest first");
            for (i, (tier, bytes)) in t.iter().enumerate() {
                let fits = tier_fits(room, core::str::from_utf8(tier).unwrap(), *bytes, total);
                assert_eq!(fits == Some(true), o.contains(&(i as u8)), "{total} {who:?} {i}");
            }
        }
        assert!(offered(&t, Some(total), For::NoDisk).is_empty());
    }
    /* Memory the kernel would not report fits nothing. */
    assert!(offered(&t, None, For::Installed).is_empty());
}

#[test]
fn the_session_keeps_the_larger_of_1_gib_and_a_quarter() {
    assert_eq!(session_keep(2 * GIB), GIB);
    assert_eq!(session_keep(4 * GIB), GIB);
    assert_eq!(session_keep(8 * GIB), 2 * GIB);
    assert_eq!(session_keep(64 * GIB), 16 * GIB);
}

/* Every tier a session offers, an install on the same machine offers too. */
#[test]
fn a_session_never_offers_more_than_an_install() {
    let t = tiers();
    for total in [2 * GIB, 3 * GIB, 4 * GIB, 8 * GIB, 16 * GIB, 64 * GIB] {
        let (disk, mem) =
            (offered(&t, Some(total), For::Installed), offered(&t, Some(total), For::Session));
        assert!(mem.iter().all(|i| disk.contains(i)), "{total}");
    }
}

/*
 * The stick tier: Qwen3 0.6B, 639 MB of file and 0.98 GB to run. An
 * installed 2 GiB machine runs it with 1 GiB left; a live 2 GiB session
 * cannot also hold its file in memory, a 3 GiB one can.
 */
#[test]
fn the_stick_tier_is_offered_where_it_fits_and_starts_the_step() {
    let t = tiers();
    let need = memory(STICK_TIER, 639_446_688).unwrap();
    assert!(need > 900_000_000 && need < 1_000_000_000, "{need}");
    let two = offered(&t, Some(2 * GIB), For::Installed);
    assert!(names(&t, &two).contains(&STICK_TIER));
    assert_eq!(chosen(&t, &two, 2 * GIB, For::Installed), Some(STICK_TIER));
    let live_two = offered(&t, Some(2 * GIB), For::Session);
    assert!(!names(&t, &live_two).contains(&STICK_TIER), "{:?}", names(&t, &live_two));
    assert_eq!(
        chosen(&t, &live_two, 2 * GIB, For::Session),
        live_two.last().map(|&i| core::str::from_utf8(t[i as usize].0).unwrap())
    );
    for total in [3 * GIB, 4 * GIB, 8 * GIB, 64 * GIB] {
        for who in [For::Installed, For::Session] {
            let o = offered(&t, Some(total), who);
            assert_eq!(chosen(&t, &o, total, who), Some(STICK_TIER), "{total} {who:?}");
        }
    }
}

#[test]
fn no_disk_or_nothing_fitting_starts_on_none() {
    let t = tiers();
    assert_eq!(
        default_row(&t, &offered(&t, Some(64 * GIB), For::NoDisk), Some(64 * GIB), For::NoDisk),
        0
    );
    assert_eq!(default_row(&t, &[], Some(GIB), For::Installed), 0);
    /* Where only Qwen2.5 0.5B fits an install, the stick tier does not: the largest that fits starts. */
    let small = offered(&t, Some(1_800_000_000), For::Installed);
    assert_eq!(names(&t, &small), ["small"]);
    assert_eq!(chosen(&t, &small, 1_800_000_000, For::Installed), Some("small"));
    /* A live 2 GiB session holds no tier at all, so none. */
    assert_eq!(chosen(&t, &offered(&t, Some(2 * GIB), For::Session), 2 * GIB, For::Session), None);
}

/* What an 8 GiB machine offers, and the count the step's line gives for the rest. */
#[test]
fn an_eight_gigabyte_machine_lists_up_to_8b_and_counts_the_rest() {
    let t = tiers();
    let o = offered(&t, Some(8 * GIB), For::Installed);
    let n = names(&t, &o);
    assert!(n.contains(&"qwen3-4b") && n.contains(&"qwen3-8b"), "{n:?}");
    assert!(!n.contains(&"qwen3-14b") && !n.contains(&"qwen3-32b"), "{n:?}");
    let hidden = t.len() - o.len();
    assert!(hidden > 0 && hidden < t.len(), "{hidden}");
}
