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

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use super::model::Domain;
use super::quarantine_rule::{add, meets};
use super::rule::{admit, admit_surface, Facts, ImportError, DEVICE_WRITES, MAX_IMPORTS};
use super::runs::{map_runs, runs, Mapped, Space, Step, PAGE};
use super::table_rule::{held_by, take_one, take_where, Import, UnimportError};

const IOVA: u64 = 0x10_0000;

fn ok_claim(iommu: bool) -> Facts {
    Facts { claim: Some((true, true)), iommu }
}

/// A surface's frames, scattered as user pages are: two runs and a lone page.
fn frames() -> Vec<u64> {
    vec![0x8_0000, 0x8_1000, 0x8_2000, 0x20_0000, 0x20_1000, 0x5_0000]
}

fn import(id: u64, pid: u32, device_id: u64, handle: u64) -> Import {
    let r = runs(frames().into_iter());
    let length = frames().len() as u64 * PAGE;
    Import { id, pid, device_id, epoch: 1, handle, iova: IOVA + id * 0x10_0000, length, runs: r }
}

#[test]
fn refused_without_an_iommu_in_service() {
    assert_eq!(admit(ok_claim(false)), Err(ImportError::Unconfined));
    assert_eq!(admit(ok_claim(true)), Ok(()));
}

#[test]
fn the_device_side_is_asked_before_the_unit() {
    let none = Facts { claim: None, iommu: false };
    assert_eq!(admit(none), Err(ImportError::NotClaimed));
    let other = Facts { claim: Some((false, true)), iommu: true };
    assert_eq!(admit(other), Err(ImportError::NotClaimed));
    let stale = Facts { claim: Some((true, false)), iommu: false };
    assert_eq!(admit(stale), Err(ImportError::StaleEpoch));
}

#[test]
fn refused_for_a_surface_the_caller_does_not_hold_attached() {
    assert_eq!(admit_surface(false, 6, 0), Err(ImportError::NotAttached));
    // Unattached is said before anything about the surface itself is.
    assert_eq!(admit_surface(false, 0, MAX_IMPORTS), Err(ImportError::NotAttached));
    assert_eq!(admit_surface(true, 0, 0), Err(ImportError::BadHandle));
    assert_eq!(admit_surface(true, 6, MAX_IMPORTS), Err(ImportError::TooMany));
    assert_eq!(admit_surface(true, 6, MAX_IMPORTS - 1), Ok(()));
}

#[test]
fn frames_fold_into_runs_in_page_order() {
    let r = runs(frames().into_iter());
    assert_eq!(r, [(0x8_0000, 3 * PAGE), (0x20_0000, 2 * PAGE), (0x5_0000, PAGE)]);
    // Adjacent but out of order is two runs: page order is the device's.
    assert_eq!(runs([0x2000u64, 0x1000].into_iter()), [(0x2000, PAGE), (0x1000, PAGE)]);
    assert!(runs(core::iter::empty()).is_empty());
}

#[test]
fn every_page_is_mapped_read_only_at_its_place_in_one_run() {
    let mut d = Domain::default();
    let r = runs(frames().into_iter());
    assert_eq!(map_runs(&mut d, IOVA, &r), Mapped::All);
    assert!(!DEVICE_WRITES);
    for (i, f) in frames().iter().enumerate() {
        let at = IOVA + i as u64 * PAGE;
        assert_eq!(d.pages.get(&at), Some(&(*f, true, false)), "page {i}");
    }
    assert_eq!(d.pages.len(), frames().len());
}

#[test]
fn a_refused_run_takes_back_the_runs_before_it() {
    let mut d = Domain { fail_map: Some((2, Step::Refused)), ..Domain::default() };
    let r = runs(frames().into_iter());
    assert_eq!(map_runs(&mut d, IOVA, &r), Mapped::Undone);
    assert!(d.pages.is_empty() && d.cached.is_empty(), "nothing left for the device");
    assert_eq!(d.invalidations, 1);
}

#[test]
fn an_unconfirmed_map_is_taken_back_or_stranded() {
    let r = runs(frames().into_iter());
    let mut d = Domain { fail_map: Some((1, Step::Uncommitted)), ..Domain::default() };
    assert_eq!(map_runs(&mut d, IOVA, &r), Mapped::Undone);
    assert!(d.pages.is_empty() && d.cached.is_empty());
    let mut stuck = Domain { fail_map: Some((1, Step::Uncommitted)), stuck: true, ..Domain::default() };
    assert_eq!(map_runs(&mut stuck, IOVA, &r), Mapped::Stranded);
    let mut stranded = Domain { fail_map: Some((0, Step::Stranded)), ..Domain::default() };
    assert_eq!(map_runs(&mut stranded, IOVA, &r), Mapped::Stranded);
}

#[test]
fn a_stranded_surface_is_quarantined_whole() {
    let r = runs(frames().into_iter());
    let mut held = Vec::new();
    for &(p, l) in &r {
        add(&mut held, p, l);
    }
    for f in frames() {
        assert!(meets(&held, f, PAGE), "frame {f:#x} could be handed out again");
    }
    assert!(meets(&held, 0x7_f000, 2 * PAGE), "a run that overlaps is held");
    assert!(!meets(&held, 0x7_f000, PAGE));
    assert!(!meets(&held, 0x8_3000, PAGE));
    add(&mut held, 0x9_0000, 0);
    assert_eq!(held.len(), r.len(), "an empty run adds nothing");
}

#[test]
fn unimport_unmaps_and_invalidates_before_the_record_is_gone() {
    let mut d = Domain::default();
    let mut list = vec![import(1, 7, 40, 0x11)];
    assert_eq!(map_runs(&mut d, list[0].iova, &list[0].runs), Mapped::All);
    assert_eq!(take_one(&mut list, 8, 1), Err(UnimportError::NotHolder));
    assert_eq!(take_one(&mut list, 7, 9), Err(UnimportError::Unknown));
    let gone = take_one(&mut list, 7, 1).expect("the holder's own import");
    assert!(d.unmap(gone.iova, gone.length));
    assert!(list.is_empty());
    for i in 0..frames().len() as u64 {
        assert!(!d.reaches(gone.iova + i * PAGE), "IOTLB still holds page {i}");
    }
    assert_eq!(d.invalidations, 1);
}

#[test]
fn exit_device_release_and_attach_release_each_drain_their_own() {
    let mut list = vec![
        import(1, 7, 40, 0x11),
        import(2, 7, 40, 0x12),
        import(3, 7, 41, 0x11),
        import(4, 9, 40, 0x11),
    ];
    assert_eq!(held_by(&list, 7), 3);
    let attach = take_where(&mut list, |i| i.pid == 7 && i.handle == 0x12);
    assert_eq!(attach.iter().map(|i| i.id).collect::<Vec<_>>(), [2]);
    let device = take_where(&mut list, |i| i.pid == 7 && i.device_id == 41);
    assert_eq!(device.iter().map(|i| i.id).collect::<Vec<_>>(), [3]);
    let exit = take_where(&mut list, |i| i.pid == 7);
    assert_eq!(exit.iter().map(|i| i.id).collect::<Vec<_>>(), [1]);
    assert_eq!(list.iter().map(|i| i.id).collect::<Vec<_>>(), [4], "another holder's stays");
}

#[test]
fn an_exit_leaves_nothing_in_the_domain() {
    let mut d = Domain::default();
    let mut list = vec![import(1, 7, 40, 0x11), import(2, 7, 40, 0x12)];
    for i in &list {
        assert_eq!(map_runs(&mut d, i.iova, &i.runs), Mapped::All);
    }
    for i in take_where(&mut list, |i| i.pid == 7) {
        assert!(d.unmap(i.iova, i.length));
    }
    assert!(d.pages.is_empty() && d.cached.is_empty());
    assert_eq!(d.invalidations, 2);
}
