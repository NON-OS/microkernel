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
 * Uninstalling a Qwen tier: which of its files go, and what each way the
 * fetcher's `remove` can end reads as. Every file of the tier goes, never
 * one another tier still needs, and nothing that cannot be removed reads
 * as removed.
 */

use std::collections::BTreeMap;

use crate::install::fetch_exit::{
    BUSY, DONE, LOCKED, NO_MEMORY, NO_VOLUME, REFUSED, USAGE, VOLUME_FAILED,
};
use crate::install::model_remove::{remove_not_started, removed};
use crate::install::why::Why;
use crate::pins::all;
use crate::remove_plan::plan;
use crate::store::progress::Progress;

fn pins() -> Vec<(&'static str, &'static [u8])> {
    all().map(|p| (p.tier, p.name)).collect()
}

#[test]
fn no_model_file_is_pinned_under_two_tiers() {
    let mut seen: BTreeMap<&[u8], &str> = BTreeMap::new();
    for (tier, name) in pins() {
        if let Some(other) = seen.insert(name, tier) {
            panic!("{} pinned under {other} and {tier}", String::from_utf8_lossy(name));
        }
    }
}

#[test]
fn removing_a_tier_takes_every_file_pinned_under_it() {
    let pins = pins();
    for (tier, _) in &pins {
        let mut want: Vec<&[u8]> =
            pins.iter().filter(|(t, _)| t == tier).map(|(_, n)| *n).collect();
        want.dedup();
        assert_eq!(plan(tier, &pins, |_| true), want, "{tier}");
    }
}

#[test]
fn a_file_another_whole_tier_pins_stays() {
    let pins: Vec<(&str, &[u8])> =
        vec![("a", b"/shared.gguf"), ("a", b"/a.gguf"), ("b", b"/shared.gguf")];
    assert_eq!(plan("a", &pins, |t| t == "b"), vec![&b"/a.gguf"[..]]);
    // A tier not on the volume needs nothing kept for it.
    assert_eq!(plan("a", &pins, |_| false), vec![&b"/shared.gguf"[..], &b"/a.gguf"[..]]);
    assert!(plan("none", &pins, |_| true).is_empty());
}

#[test]
fn only_a_whole_removal_reads_as_removed() {
    assert_eq!(removed(i64::from(DONE)), Ok(()));
    // No disk carries NONOS, so no volume and no model on one.
    assert_eq!(removed(i64::from(NO_VOLUME)), Ok(()));
    for status in -300..=300 {
        if status != i64::from(DONE) && status != i64::from(NO_VOLUME) {
            assert!(removed(status).is_err(), "status {status}");
        }
    }
}

#[test]
fn each_way_a_removal_stops_is_its_own_reason() {
    assert_eq!(removed(i64::from(BUSY)), Err(Why::ModelDownloading));
    assert_eq!(removed(i64::from(LOCKED)), Err(Why::VolumeLocked));
    assert_eq!(removed(i64::from(VOLUME_FAILED)), Err(Why::VolumeFailed));
    assert_eq!(removed(i64::from(NO_MEMORY)), Err(Why::NoMemory));
    assert_eq!(removed(i64::from(USAGE)), Err(Why::ModelUnknown));
    assert_eq!(removed(i64::from(REFUSED)), Err(Why::RemovePartial));
    // Built without the fetcher: nothing can take the model off the volume.
    assert_eq!(remove_not_started(-2), Why::ModelKept);
    assert_eq!(remove_not_started(-16), Why::ModelBusy);
    assert_eq!(remove_not_started(-12), Why::RemovePartial);
    /*
     * A boot with no network never starts the fetcher: nothing was tried,
     * so it is not "some files would not go", and no retry is offered.
     */
    assert_eq!(remove_not_started(-100), Why::BootOffline);
    assert!(!Progress::Failed(Why::BootOffline.code() as u8).retryable());
}

#[test]
fn a_removal_that_can_finish_offers_a_retry_and_one_that_cannot_does_not() {
    let retry = |why: Why| Progress::Failed(why.code() as u8).retryable();
    assert!(retry(Why::ModelDownloading) && retry(Why::RemovePartial));
    assert!(!retry(Why::ModelKept) && !retry(Why::NotInstalled));
}
