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

/*
 * The store's tier cards against the one fit rule: a tier card is shown
 * exactly when the fetcher would download the tier (`too_large` is None),
 * the tiers come smallest first after every other listing, and the line
 * counting the rest says how many are not shown.
 */

use std::collections::BTreeMap;

use crate::memory_need::too_large;
use crate::memory_tests::need;
use crate::need::{Room, STICK_TIER};
use crate::pins::all;
use crate::store::tier_fit::{hidden_line, order, shown, tier_of};

const GIB: u64 = 1 << 30;

fn weights() -> Vec<(&'static str, u64)> {
    let mut sum: BTreeMap<&'static str, u64> = BTreeMap::new();
    for p in all() {
        *sum.entry(p.tier).or_default() += p.bytes;
    }
    sum.into_iter().collect()
}

#[test]
fn a_card_is_shown_exactly_when_the_fetcher_would_download_it() {
    let w = weights();
    for total in [2 * GIB, 3 * GIB, 4 * GIB, 8 * GIB, 16 * GIB, 32 * GIB, 64 * GIB] {
        for room in [Room::Disk, Room::Memory] {
            for &(tier, bytes) in &w {
                let id = format!("linux.qwen-{tier}");
                let fetches = too_large(tier, bytes, need(tier), Some(total), room).is_none();
                assert_eq!(shown(id.as_bytes(), &w, Some(total), room), fetches, "{tier} {total} {room:?}");
            }
        }
    }
}

#[test]
fn other_listings_and_unknown_memory_hide_nothing() {
    let w = weights();
    for id in ["linux.jq", "nonos.terminal", "linux.qwen-giant", "linux.qwen-"] {
        assert!(shown(id.as_bytes(), &w, Some(1), Room::Memory), "{id}");
    }
    assert!(shown(b"linux.qwen-max", &w, None, Room::Memory));
    assert_eq!(tier_of(b"linux.qwen-"), None);
    assert_eq!(tier_of(b"linux.qwen-qwen3-4b"), Some("qwen3-4b"));
}

#[test]
fn tiers_sort_smallest_first_after_everything_else() {
    let w = weights();
    let mut ids = vec!["linux.qwen-qwen3-4b", "linux.jq", "linux.qwen-small", "linux.qwen-qwen3-0.6b"];
    ids.sort_by_key(|id| order(id.as_bytes(), &w));
    assert_eq!(ids, ["linux.jq", "linux.qwen-small", "linux.qwen-qwen3-0.6b", "linux.qwen-qwen3-4b"]);
}

/* The personality's installer names the stick tier the fetcher and the store name. */
#[test]
fn the_installer_and_the_fetcher_name_one_stick_tier() {
    assert_eq!(crate::install::apps::STICK_TIER, STICK_TIER);
    assert!(crate::install::apps::app(&format!("qwen-{STICK_TIER}")).is_some());
}

/* On a live 3 GiB stick the stick tier is a card; an 8 GiB install shows up to 8B. */
#[test]
fn the_stick_tier_is_a_card_where_it_fits_and_the_rest_are_counted() {
    let w = weights();
    let stick = format!("linux.qwen-{STICK_TIER}");
    assert!(shown(stick.as_bytes(), &w, Some(3 * GIB), Room::Memory));
    assert!(!shown(stick.as_bytes(), &w, Some(2 * GIB), Room::Memory));
    let hidden = w
        .iter()
        .filter(|(t, _)| !shown(format!("linux.qwen-{t}").as_bytes(), &w, Some(8 * GIB), Room::Disk))
        .count();
    assert!(hidden > 0 && hidden < 17, "{hidden}");
    let line = hidden_line(hidden).unwrap();
    assert_eq!(line, format!("{hidden} larger tiers need more memory than this machine has"));
    assert_eq!(hidden_line(0), None);
    assert_eq!(hidden_line(1).unwrap(), "1 larger tier needs more memory than this machine has");
}
