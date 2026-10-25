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

//! The presets as named on the command line, and every form refused.

use crate::boxed::folders::may_write;
use crate::boxed::preset::{parse, Preset, OWN};
use nonos_linux_policy::mask::{EXEC, FILES_HOME, NET_ANON, TTY};

#[test]
fn strict_is_one_scratch_folder_and_no_network() {
    let p = parse(b"strict").unwrap();
    assert_eq!((p.preset, p.mask), (Preset::Strict, TTY | EXEC));
    assert_eq!(p.folders, [b"/tmp".to_vec()]);
    assert!(p.dests.is_empty());
    assert!(may_write(&p.folders, b"/tmp/out.txt") && may_write(&p.folders, b"/tmp"));
    for refused in [&b"/tmpx/a"[..], b"/home/a", b"/etc/passwd", b"/run/a", b"/"] {
        assert!(!may_write(&p.folders, refused), "{}", String::from_utf8_lossy(refused));
    }
}

#[test]
fn offline_leaves_the_runs_own_folders_and_no_network() {
    let p = parse(b"offline").unwrap();
    assert_eq!((p.preset, p.mask), (Preset::Offline, TTY | EXEC | FILES_HOME));
    assert_eq!(p.folders.len(), OWN.len());
    assert!(may_write(&p.folders, b"/home/user/notes") && !may_write(&p.folders, b"/usr/bin/x"));
}

#[test]
fn allow_names_its_folders_and_destinations_once_and_sorted() {
    let p = parse(b"allow:/tmp/b,net:example.com:443,/tmp/a,/tmp/b,net:10.0.0.1:80").unwrap();
    assert_eq!(p.preset, Preset::Allow);
    assert_eq!(p.folders, [b"/tmp/a".to_vec(), b"/tmp/b".to_vec()]);
    assert_eq!(p.mask, TTY | EXEC | FILES_HOME | NET_ANON);
    assert_eq!(nonos_linux_policy::dests::check(&p.dests), Ok(2));
    let folders_only = parse(b"allow:/tmp/a").unwrap();
    assert!(folders_only.dests.is_empty() && folders_only.mask & NET_ANON == 0);
}

#[test]
fn a_name_is_held_as_the_list_holds_it() {
    let p = parse(b"allow:net:Example.COM:443").unwrap();
    let d = nonos_linux_policy::dests::entries(&p.dests).next().unwrap();
    assert_eq!((d.body, d.first_port, d.last_port), (&b"example.com"[..], 443, 443));
}
