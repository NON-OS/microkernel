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

//! C10c: a member name with a `..` component, or absolute (ustar's prefix
//! included), fails its package; so does one of the personality's own paths;
//! and a path another installed package holds is a conflict.

use super::tar_build::head;
use crate::install::manifest::Manifest;
use crate::install::member::conflict::held_by_another;
use crate::install::member::name::{member_refused, Refused};
use crate::install::tar::walk;

#[test]
fn a_dotdot_component_or_an_absolute_name_fails_the_package() {
    for n in [&b"x/../../etc/nonos-links"[..], b"..", b"usr/..", b"a/../b"] {
        assert_eq!(member_refused(n), Some(Refused::Escapes), "{:?}", core::str::from_utf8(n));
    }
    assert_eq!(member_refused(b"/etc/passwd"), Some(Refused::Absolute));
    let w = walk(&head("x", "0", b'5', "", Some("/etc")));
    assert_eq!(member_refused(&w.entries[0].name), Some(Refused::Absolute), "ustar's prefix");
}

#[test]
fn the_personality_s_own_paths_are_refused_however_they_are_spelled() {
    for n in [
        &b"etc/nonos-links"[..],
        b"etc//nonos-links",
        b"etc/./nonos-boot-guest",
        b"etc/nonos-install-max",
        b"usr/share/nonos",
        b"usr/share/nonos/keymap.xkb",
    ] {
        assert_eq!(member_refused(n), Some(Refused::Reserved), "{:?}", core::str::from_utf8(n));
    }
    for n in [&b"usr/bin/jq"[..], b"etc/nonos-links.d/x", b"usr/share/nonos-extra/x", b"a/b..c"] {
        assert_eq!(member_refused(n), None, "{:?}", core::str::from_utf8(n));
    }
}

#[test]
fn a_path_another_package_holds_is_a_conflict() {
    let a = Manifest { files: vec![b"/bin/ls".to_vec()], links: vec![b"/usr/bin/vi".to_vec()] };
    let mine = |p: &[&[u8]]| p.iter().map(|x| x.to_vec()).collect::<Vec<_>>();
    assert_eq!(
        held_by_another(&mine(&[b"/usr/bin/jq", b"/bin/ls"]), core::slice::from_ref(&a)),
        Some(&b"/bin/ls"[..])
    );
    assert_eq!(
        held_by_another(&mine(&[b"/usr/bin/vi"]), core::slice::from_ref(&a)),
        Some(&b"/usr/bin/vi"[..])
    );
    assert_eq!(held_by_another(&mine(&[b"/usr/bin/jq"]), core::slice::from_ref(&a)), None);
    assert_eq!(held_by_another(&mine(&[b"/bin/ls"]), &[]), None, "no other package");
}
