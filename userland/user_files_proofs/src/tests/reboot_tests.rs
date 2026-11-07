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

//! A reboot: the volume kept, everything in RAM gone.

use nonos_user_files::errno::{EIO, ENOENT, EUCLEAN};
use nonos_user_files::limits::DEFAULT_QUOTA;
use nonos_user_files::wire::flags::{CREATE, READ, WRITE};
use nonos_user_files::Broker;

use super::live::no_strays;
use super::{fresh, get_file, pattern, put_file, settle, APP};
use crate::doubles::{client, Volume};

#[test]
fn the_tree_and_every_byte_come_back() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    c.mkdir(b"Documents/taxes", 0o750).unwrap();
    c.mkdir(b"Documents/taxes/2026", 0o750).unwrap();
    drop(c);
    put_file(&mut b, &desk, b"Documents/taxes/2026/return.ods", &pattern(1, 70_000));
    put_file(&mut b, &desk, b"Downloads/manual.pdf", &pattern(2, 5_500_000));
    put_file(&mut b, &desk, b"Documents/empty", b"");
    settle(&mut b, &desk);
    let mut next = Broker::new(b.objects().reboot(), DEFAULT_QUOTA);
    assert_eq!(
        get_file(&mut next, &desk, b"Documents/taxes/2026/return.ods").unwrap(),
        pattern(1, 70_000)
    );
    assert_eq!(get_file(&mut next, &desk, b"Downloads/manual.pdf").unwrap(), pattern(2, 5_500_000));
    assert_eq!(get_file(&mut next, &desk, b"Documents/empty").unwrap(), b"");
    let a = client(&mut next, &desk, APP).stat(b"Documents/taxes").unwrap();
    assert_eq!(a.mode, 0o750);
    no_strays(next.objects());
}

#[test]
fn a_change_not_yet_committed_is_lost_and_a_synced_one_is_not() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    c.mkdir(b"Documents/kept", 0o700).unwrap();
    let (h, _) = c.open(b"Documents/kept/f", WRITE | CREATE, 0o600).unwrap();
    c.fsync(h).unwrap();
    c.mkdir(b"Documents/owed", 0o700).unwrap();
    drop(c);
    let mut next = Broker::new(b.objects().reboot(), DEFAULT_QUOTA);
    let mut c = client(&mut next, &desk, APP);
    assert!(c.stat(b"Documents/kept/f").is_ok());
    assert_eq!(c.stat(b"Documents/owed"), Err(ENOENT));
}

#[test]
fn a_file_unlinked_while_open_is_gone_after_a_reboot() {
    let (mut b, desk) = fresh();
    put_file(&mut b, &desk, b"Documents/held", &pattern(3, 2_500_000));
    let mut c = client(&mut b, &desk, APP);
    let (h, _) = c.open(b"Documents/held", READ, 0).unwrap();
    c.unlink(b"Documents/held").unwrap();
    let mut buf = vec![0u8; 100];
    assert_eq!(c.read(h, 2_000_000, &mut buf).unwrap(), 100);
    assert_eq!(buf, pattern(3, 2_500_000)[2_000_000..2_000_100]);
    drop(c);
    settle(&mut b, &desk);
    let mut next = Broker::new(b.objects().reboot(), DEFAULT_QUOTA);
    assert_eq!(client(&mut next, &desk, APP).stat(b"Documents/held"), Err(ENOENT));
    no_strays(next.objects());
}

#[test]
fn two_broken_slots_refuse_to_start_and_touch_nothing() {
    let (mut b, desk) = fresh();
    put_file(&mut b, &desk, b"Documents/a", b"a");
    put_file(&mut b, &desk, b"Documents/b", b"b");
    let mut v = b.objects().reboot();
    for slot in [0u64, 1] {
        let s = v.objects.get_mut(&slot).unwrap();
        let mid = s.len() / 2;
        s[mid] ^= 0xff;
    }
    let held = v.objects.clone();
    let mut next = Broker::new(v, DEFAULT_QUOTA);
    assert_eq!(client(&mut next, &desk, APP).stat(b"Documents"), Err(EUCLEAN));
    assert_eq!(next.objects().objects, held, "a broker that could not start removed something");
}

#[test]
fn a_volume_that_cannot_be_read_is_asked_again_later() {
    let (mut b, desk) = fresh();
    put_file(&mut b, &desk, b"Documents/a", b"abc");
    let mut v = b.objects().reboot();
    v.cut = Some(0);
    let mut next = Broker::new(v, DEFAULT_QUOTA);
    assert_eq!(client(&mut next, &desk, APP).stat(b"Documents/a"), Err(EIO));
    next.objects().cut = None;
    assert_eq!(client(&mut next, &desk, APP).stat(b"Documents/a").unwrap().size, 3);
}

#[test]
fn a_fresh_volume_is_written_only_when_something_changes() {
    let (mut b, desk) = fresh();
    let _ = client(&mut b, &desk, APP).stat(b"Documents").unwrap();
    settle(&mut b, &desk);
    assert!(b.objects().objects.is_empty());
    let _: &Volume = b.objects();
}

#[test]
fn a_wall_clock_set_back_does_not_hold_a_commit() {
    let (mut b, desk) = fresh();
    client(&mut b, &desk, APP).mkdir(b"Documents/owed", 0o700).unwrap();
    desk.wall_back.set(3_600_000_000_000);
    desk.step(1_000_000_000);
    b.tick(desk.now.get()).unwrap();
    let mut next = Broker::new(b.objects().reboot(), DEFAULT_QUOTA);
    assert!(client(&mut next, &desk, APP).stat(b"Documents/owed").is_ok());
}

#[test]
fn hundreds_of_commits_journal_then_checkpoint_and_come_back_whole() {
    use nonos_user_files::manifest::{Slot, JOURNAL_MAX};
    let (mut b, desk) = fresh();
    let mut longest = 0;
    for i in 0..300u32 {
        client(&mut b, &desk, APP).mkdir(format!("Documents/d{i:03}").as_bytes(), 0o755).unwrap();
        if i % 3 == 0 {
            put_file(
                &mut b,
                &desk,
                format!("Downloads/f{i:03}").as_bytes(),
                &pattern(i as u8, 3000),
            );
        }
        settle(&mut b, &desk);
        let v = b.objects();
        let newest =
            [0u64, 1].iter().filter_map(|id| v.objects.get(id).and_then(|r| Slot::decode(r)));
        let journal = newest.max_by_key(|s| s.generation).unwrap().journal.len();
        assert!(journal <= JOURNAL_MAX);
        longest = longest.max(journal);
        if i % 37 == 36 {
            no_strays(b.objects());
            b = Broker::new(b.objects().reboot(), DEFAULT_QUOTA);
            let l = client(&mut b, &desk, APP).list(b"Documents", 0, 512).unwrap();
            assert_eq!(l.entries.len(), i as usize + 1);
        }
    }
    assert!(longest > 8, "the journal never grew: {longest}");
    let mut next = Broker::new(b.objects().reboot(), DEFAULT_QUOTA);
    assert_eq!(get_file(&mut next, &desk, b"Downloads/f297").unwrap(), pattern(297u32 as u8, 3000));
    no_strays(next.objects());
}

/// A volume committed when the root held only Documents and Downloads, as
/// the first broker wrote it: one file in Documents.
fn two_folder_volume() -> Volume {
    use nonos_user_files::manifest::{body_hash, decode, encode, first_body, Slot};
    let mut d = decode(&first_body()).unwrap();
    let root = d.tree.nodes.get_mut(&nonos_user_files::ROOT).unwrap();
    let later: Vec<u64> = [&b"Music"[..], b"Pictures", b"Videos"]
        .iter()
        .map(|n| root.children.remove(*n).unwrap())
        .collect();
    for ino in later {
        d.tree.nodes.remove(&ino);
    }
    let body = encode(&d.tree, 3, &[]);
    let slot = Slot {
        generation: 1,
        body_len: body.len() as u64,
        body_hash: body_hash(&body),
        parts: vec![2],
        journal: Vec::new(),
    };
    let mut v = Volume::default();
    v.objects.insert(2, body);
    v.objects.insert(1, slot.encode());
    v
}

#[test]
fn a_volume_from_before_the_later_folders_gains_them_and_keeps_its_files() {
    let desk = crate::doubles::Desk::new(&[APP]);
    let mut b = Broker::new(two_folder_volume(), DEFAULT_QUOTA);
    let root = client(&mut b, &desk, APP).list(b"", 0, 64).unwrap();
    let names: Vec<Vec<u8>> = root.entries.into_iter().map(|e| e.2).collect();
    assert_eq!(names, [&b"Documents"[..], b"Downloads", b"Music", b"Pictures", b"Videos"]);
    put_file(&mut b, &desk, b"Pictures/cat.png", b"png");
    settle(&mut b, &desk);
    let mut next = Broker::new(b.objects().reboot(), DEFAULT_QUOTA);
    assert_eq!(get_file(&mut next, &desk, b"Pictures/cat.png").unwrap(), b"png");
    no_strays(next.objects());
}
