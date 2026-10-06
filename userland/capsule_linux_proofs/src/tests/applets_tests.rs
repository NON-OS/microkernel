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

//! The built-in BusyBox's program table, read from the pinned binary the
//! personality embeds: what `linux <name>` runs when the store holds no
//! Linux tree.

use crate::applets::{has, names};

static BUSYBOX: &[u8] = include_bytes!("../../../capsule_linux/guests/busybox.elf");

#[test]
fn the_pinned_busybox_holds_its_whole_sorted_table() {
    let all: Vec<&[u8]> = names(BUSYBOX).collect();
    assert_eq!(all.len(), 305, "BusyBox 1.36.1 as configured in busybox.config");
    assert!(all.windows(2).all(|w| w[0] < w[1]), "BusyBox's own table is sorted");
    assert_eq!(all.first(), Some(&&b"["[..]));
    assert_eq!(all.last(), Some(&&b"zcip"[..]));
}

#[test]
fn the_everyday_programs_are_there_and_others_are_not() {
    for name in ["sh", "ash", "ls", "cat", "vi", "grep", "sed", "awk", "tar", "wget", "top", "ps"] {
        assert!(has(BUSYBOX, name.as_bytes()), "{name}");
    }
    for name in ["busybox", "python3", "bash", "", "l", "lss"] {
        assert!(!has(BUSYBOX, name.as_bytes()), "{name:?} is not one of its programs");
    }
}

#[test]
fn bytes_without_a_table_name_nothing() {
    assert_eq!(names(b"\x7fELF no table here").count(), 0);
    assert!(!has(b"", b"ls"));
}

/*
 * What the tools carried in the store reach by name, through PATH, and find
 * only because a bin directory shows each built-in program as a file
 * (file/meta/node/path.rs): make runs `echo` itself, a shell `mkdir` and
 * `touch`, perl's system() and python's subprocess `sh`. Gate boot 2 had
 * make saying "echo: No such file" and sh "mkdir: not found".
 */
#[test]
fn the_programs_the_stored_tools_reach_by_name_are_built_in() {
    for name in [&b"sh"[..], b"echo", b"mkdir", b"touch", b"cat", b"ls", b"rm", b"cp", b"mv", b"env", b"test", b"printf", b"sed", b"grep"] {
        assert!(has(BUSYBOX, name), "{}", String::from_utf8_lossy(name));
    }
}
