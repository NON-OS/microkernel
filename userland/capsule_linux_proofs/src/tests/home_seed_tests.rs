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

//! A home seeded once (capsule_linux file/private/seed_rule.rs): written the
//! first time, kept on a second start, a person's edit never overwritten, a
//! full home refused cleanly and finished at the next start, and an image's
//! INDEX taken whole or not at all.

use std::collections::{BTreeMap, BTreeSet};

use crate::seed_rule::{index, path_ok, plant, Fault, Home, Planted, MARK, MOST_DEPTH, MOST_FILES};
use crate::seed_rule::{USER_DIRS, USER_DIRS_TEXT};

/// A home in memory that runs out of room after `room` writes when set.
#[derive(Default)]
struct Mem {
    files: BTreeMap<Vec<u8>, Vec<u8>>,
    dirs: BTreeSet<Vec<u8>>,
    room: Option<usize>,
    refuse: bool,
}

impl Home for Mem {
    fn has(&mut self, p: &[u8]) -> Result<bool, Fault> {
        Ok(self.files.contains_key(p) || self.dirs.contains(p))
    }
    fn mkdir(&mut self, p: &[u8]) -> Result<(), Fault> {
        self.dirs.insert(p.to_vec());
        Ok(())
    }
    fn write(&mut self, p: &[u8], b: &[u8]) -> Result<(), Fault> {
        if self.refuse {
            return Err(Fault::Refused);
        }
        match self.room {
            Some(0) => return Err(Fault::Full),
            Some(n) => self.room = Some(n - 1),
            None => {}
        }
        let parent = p.iter().rposition(|c| *c == b'/').map(|i| &p[..i]);
        assert!(parent.is_none_or(|d| self.dirs.contains(d)), "a file before its directory");
        self.files.insert(p.to_vec(), b.to_vec());
        Ok(())
    }
}

const XCU: &[u8] = b".config/libreoffice/4/user/registrymodifications.xcu";

fn seed() -> Vec<(&'static [u8], &'static [u8])> {
    vec![(USER_DIRS, USER_DIRS_TEXT), (XCU, b"<items/>"), (b"notes.txt", b"hello")]
}

#[test]
fn a_new_home_is_seeded_whole_and_marked() {
    let mut home = Mem::default();
    assert_eq!(plant(&mut home, &seed()), Ok(Planted::Now { written: 3, kept: 0 }));
    assert_eq!(home.files[USER_DIRS], USER_DIRS_TEXT);
    assert_eq!(home.files[XCU], b"<items/>");
    assert!(home.files.contains_key(MARK));
    assert!(home.dirs.contains(&b".config/libreoffice/4/user"[..]));
}

#[test]
fn a_second_start_writes_nothing_and_an_edit_or_removal_stays() {
    let mut home = Mem::default();
    plant(&mut home, &seed()).unwrap();
    home.files.insert(XCU.to_vec(), b"<person's own/>".to_vec());
    home.files.remove(&b"notes.txt"[..]);
    let before = home.files.clone();
    assert_eq!(plant(&mut home, &seed()), Ok(Planted::Before));
    assert_eq!(home.files, before);
}

#[test]
fn a_file_the_home_already_has_is_kept() {
    let mut home = Mem::default();
    home.dirs.insert(b".config".to_vec());
    home.files.insert(USER_DIRS.to_vec(), b"XDG_DOCUMENTS_DIR=\"$HOME/Mine\"\n".to_vec());
    assert_eq!(plant(&mut home, &seed()), Ok(Planted::Now { written: 2, kept: 1 }));
    assert_eq!(home.files[USER_DIRS], b"XDG_DOCUMENTS_DIR=\"$HOME/Mine\"\n");
}

#[test]
fn a_full_home_is_refused_cleanly_and_finished_at_the_next_start() {
    for room in 0..4 {
        let mut home = Mem { room: Some(room), ..Mem::default() };
        assert_eq!(plant(&mut home, &seed()), Err(Fault::Full), "room for {room}");
        assert!(!home.files.contains_key(MARK), "a mark on a seed cut short");
        let landed = home.files.clone();
        home.room = None;
        let Ok(Planted::Now { written, kept }) = plant(&mut home, &seed()) else { panic!() };
        assert_eq!((written, kept), (3 - landed.len(), landed.len()));
        assert!(landed.iter().all(|(p, b)| home.files[p] == *b), "a landed file written again");
        assert!(home.files.contains_key(MARK));
    }
    let mut home = Mem { refuse: true, ..Mem::default() };
    assert_eq!(plant(&mut home, &seed()), Err(Fault::Refused));
    assert!(home.files.is_empty());
}

#[test]
fn an_index_is_taken_whole_or_not_at_all() {
    let good =
        b"# LibreOffice\n.config/libreoffice/4/user/registrymodifications.xcu\r\nnotes.txt\n\n";
    assert_eq!(index(good).unwrap(), vec![XCU, &b"notes.txt"[..]]);
    for bad in [
        &b"../escape\n"[..],
        b"/etc/passwd\n",
        b"a/../../b\n",
        b"a//b\n",
        b"./a\n",
        b"a/./b\n",
        b"a\0b\n",
        b"a/b/c/d/e/f/g\n",
        b"same\nsame\n",
    ] {
        assert_eq!(index(bad), None, "{:?}", String::from_utf8_lossy(bad));
    }
    let long = [vec![b'n'; 256], b"\n".to_vec()].concat();
    assert_eq!(index(&long), None);
    let many: Vec<u8> = (0..=MOST_FILES).flat_map(|i| format!("f{i}\n").into_bytes()).collect();
    assert_eq!(index(&many), None);
    let most: Vec<u8> = (0..MOST_FILES).flat_map(|i| format!("f{i}\n").into_bytes()).collect();
    assert_eq!(index(&most).map(|v| v.len()), Some(MOST_FILES));
    let deep = ["d"; MOST_DEPTH].join("/");
    assert!(path_ok(deep.as_bytes()));
    assert!(!path_ok(b""));
}
