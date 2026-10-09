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
 * The line under a Qwen tier's card in the store: the download is the pins'
 * own length and the memory is the one the signed catalogue gives and the
 * fetcher refuses a download by, for every listing the market publishes.
 */

use std::collections::BTreeMap;

use crate::catalogue::parse::parse;
use crate::memory_tests::need;
use crate::pins::all;
use crate::size::size;
use crate::store::model_card::model_line;
use crate::tests::written;

fn weights() -> Vec<(&'static str, u64)> {
    let mut sum: BTreeMap<&'static str, u64> = BTreeMap::new();
    for p in all() {
        *sum.entry(p.tier).or_default() += p.bytes;
    }
    sum.into_iter().collect()
}

fn line(id: &str) -> Option<String> {
    model_line(id.as_bytes(), &weights()).map(|l| String::from_utf8(l).unwrap())
}

#[test]
fn the_small_and_4b_cards_say_their_download_and_memory() {
    assert_eq!(line("linux.qwen-small").unwrap(), "491 MB download, needs 613 MB of memory");
    let four = line("linux.qwen-qwen3-4b").unwrap();
    assert!(four.starts_with("2.49 GB download, needs "), "{four}");
    assert_eq!(four, format!("2.49 GB download, needs {} of memory", size(need("qwen3-4b"))));
    /* The stick tier names release sticks, not a download, and never this stick. */
    let tiny = line("linux.qwen-qwen3-0.6b").unwrap();
    assert_eq!(tiny, format!("639 MB, on release sticks; needs {} of memory", size(need("qwen3-0.6b"))));
}

#[test]
fn every_card_agrees_with_the_signed_catalogue() {
    let blob = written();
    let cat = parse(&blob[..blob.len() - 64]).unwrap();
    assert_eq!(cat.tiers.len(), 17);
    for t in &cat.tiers {
        let how = if t.word == crate::need::STICK_TIER { ", on release sticks;" } else { " download," };
        let want = format!("{}{how} needs {} of memory", size(t.bytes()), size(t.memory));
        assert_eq!(line(&format!("linux.qwen-{}", t.word)).as_deref(), Some(want.as_str()));
    }
}

#[test]
fn only_a_qwen_tier_listing_has_one() {
    for id in ["linux.jq", "linux.qwen-giant", "linux.qwen-", "nonos.terminal", "qwen-small"] {
        assert_eq!(line(id), None, "{id}");
    }
}
