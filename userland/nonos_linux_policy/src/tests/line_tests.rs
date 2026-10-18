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

//! A line of policies.txt, or of the test guests' list, as the build reads it.

#[path = "../../build/dests_arg.rs"]
mod dests_arg;
#[path = "../../build/line.rs"]
pub(super) mod line;
#[path = "../../build/mask_names.rs"]
mod mask_names;
#[path = "../../build/set_hex.rs"]
mod set_hex;

use crate::mask::{EXEC, NET_ANON, TTY, WAYLAND};
use crate::SyscallSet;
use line::parse;
use set_hex::set_of;

#[test]
fn comments_and_blanks_name_nothing() {
    assert!(["", "   ", "# /bin/q WAYLAND", "\t# x"].iter().all(|l| parse(l).unwrap().is_none()));
}

#[test]
fn a_line_names_a_program_in_bin_and_its_services() {
    let l = parse("/bin/prog  WAYLAND NET_ANON\tEXEC").unwrap().unwrap();
    assert_eq!((l.store_path, l.mask, l.file), ("/bin/prog", WAYLAND | NET_ANON | EXEC, None));
    assert_eq!(l.calls, SyscallSet::ALL);
    let guest = parse("/bin/suite file=/t/linux-guests/suite TTY").unwrap().unwrap();
    assert_eq!((guest.file, guest.mask), (Some("/t/linux-guests/suite"), TTY));
    assert!(parse("/bin/suite file=suite").is_err(), "a file= is absolute");
    for bad in ["prog", "/usr/bin/prog", "/bin/", "/bin/a/b", "/bin/prog WAYLAND NETWORK"] {
        assert!(parse(bad).is_err(), "{bad}");
    }
    assert!(parse("/bin/prog calls=00").is_err());
    assert!(parse(&format!("/bin/prog calls={}02", "00".repeat(64))).is_err(), "past 512");
}

#[test]
fn calls_read_as_the_ok_line_prints_them() {
    let mut hex = "00".repeat(65);
    hex.replace_range(56..58, "80");
    let set = set_of(&hex).unwrap();
    assert!(set.allows(231) && !set.allows(230) && !set.allows(232));
    let line = format!("/bin/prog TTY calls={hex}");
    let l = parse(&line).unwrap().unwrap();
    assert_eq!((l.mask, l.calls), (TTY, set));
    assert_eq!(set_of(&format!("{}01", "ff".repeat(64))), Some(SyscallSet::ALL));
}

#[test]
fn files_user_is_reserved_in_the_vendor_table_too() {
    let why = parse("/bin/prog WAYLAND FILES_USER").err().unwrap();
    assert!(why.contains("FILES_USER is reserved"), "{why}");
}
