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

//! Setup's answers travel to the new disk with the marker that says they
//! were kept, even from a boot whose store could not keep them, so the
//! installed system does not run setup again. Answers setup would never
//! write are not carried. The store is read back by the vfs's own decoding.

#[path = "common/fake_vfs.rs"]
mod fake_vfs;
#[path = "common/vfs/mod.rs"]
mod vfs;

/*
 * The vfs files name `alloc`, as a no_std capsule's do.
 */
extern crate alloc;

use fake_vfs::FakeVfs;
use nonos_disk::gather;
use nonos_policy_proto::setup_record::{Answers, Name, Tier, DONE};

fn kept() -> Answers {
    let (username, qwen_tier) = (Name::new(b"ada").unwrap(), Tier::new(b"small").unwrap());
    Answers { keyboard_layout: 2, timezone: -5, wallpaper: 3, username, qwen_tier }
}

#[test]
fn setup_answers_travel_with_their_marker() {
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", &kept().encode());
    let c = gather(&mut v);
    assert_eq!(c.answers, Some(kept()), "name and Qwen tier included");
    let files = vfs::load(c.store.bytes());
    let want =
        [("/nonos/setup/answers", kept().encode().to_vec()), ("/nonos/setup/done", DONE.to_vec())];
    assert_eq!(files, want.map(|(n, d)| (n.to_string(), d)));
}

#[test]
fn answers_setup_never_writes_stay_behind() {
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", b"NSA1\0\x7f\0");
    v.put("/nonos/setup/done", &DONE);
    let c = gather(&mut v);
    assert!(c.answers.is_none(), "a time zone of +127 is not one setup offers");
    assert!(vfs::load(c.store.bytes()).is_empty());
}

#[test]
fn an_older_record_travels_as_it_was_kept() {
    let mut v = FakeVfs::default();
    v.put("/nonos/setup/answers", &kept().encode_v1());
    let c = gather(&mut v);
    assert_eq!(c.answers.map(|a| a.username.as_bytes().len()), Some(0));
    assert_eq!(vfs::load(c.store.bytes())[0].1, kept().encode_v1().to_vec());
}
