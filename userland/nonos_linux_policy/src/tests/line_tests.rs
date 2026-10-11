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

//! policies.txt as the build reads it, and the committed file itself: the
//! reference app's three builds get the box plan/32 gives them.

#[path = "../../build/line.rs"]
mod line;

use crate::mask::{EXEC, MODELS_RO, NET_ANON, TTY, WAYLAND};
use crate::SyscallSet;
use line::{parse, set_of};

const COMMITTED: &str = include_str!("../../../capsule_linux/policies.txt");
const CHAT_PICK: &str = include_str!("../../../capsule_linux/src/linux/install/chat_pick.rs");

#[test]
fn comments_and_blanks_name_nothing() {
    for line in ["", "   ", "# /bin/qwenchat WAYLAND", "\t# x"] {
        assert!(parse(line).unwrap().is_none(), "{line:?}");
    }
}

#[test]
fn a_line_names_a_program_in_bin_and_its_services() {
    let l = parse("/bin/prog  WAYLAND NET_ANON\tEXEC").unwrap().unwrap();
    assert_eq!((l.store_path, l.mask), ("/bin/prog", WAYLAND | NET_ANON | EXEC));
    assert_eq!(l.calls, SyscallSet::ALL);
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
fn the_committed_file_gives_each_qwen_build_the_reference_box() {
    let lines: Vec<_> = COMMITTED.lines().filter_map(|l| parse(l).unwrap()).collect();
    let paths: Vec<_> = lines.iter().map(|l| l.store_path).collect();
    assert_eq!(paths, ["/bin/qwenchat", "/bin/qwenchat-x86_64_v2", "/bin/qwenchat-x86_64"]);
    for l in &lines {
        assert_eq!(l.mask, WAYLAND | MODELS_RO | TTY, "{}", l.store_path);
        assert!(l.mask & NET_ANON == 0, "/models is never served beside a network");
        let quoted = format!("b\"{}\"", l.store_path);
        assert!(CHAT_PICK.contains(&quoted), "chat_pick does not run {}", l.store_path);
    }
    assert!(lines.windows(2).all(|w| w[0].calls == w[1].calls), "one set for all three");
}
