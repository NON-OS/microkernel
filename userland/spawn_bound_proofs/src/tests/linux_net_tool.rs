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

//! `tool.linux.net`, the personality in the terminal role that holds
//! Network: started only for the terminal's own boxed run naming a
//! destination, and only by a caller the tool bound lets hold Network.

use crate::net_box::names_destinations;
use crate::stand_in::world;
use crate::tool_bound::check;

const LINUX: &str = include_str!("../../../capsule_linux/Capsule.mk");
const TERMINAL: &str = include_str!("../../../capsule_terminal/Capsule.mk");
const NET: u64 = 1 << 2;

fn field(mk: &str, key: &str) -> u64 {
    let line = mk.lines().find(|l| l.starts_with(key)).expect("the field");
    let hex = line.rsplit(":=").next().unwrap().trim().trim_start_matches("0x");
    u64::from_str_radix(hex, 16).expect("a hex mask")
}

fn argv(words: &[&str]) -> Vec<u8> {
    words.join("\0").into_bytes()
}

#[test]
fn only_the_terminal_s_boxed_run_naming_a_destination_is_started() {
    let key = "--box-key=00112233445566778899aabbccddeeff";
    for yes in [
        argv(&["linux", key, "--box", "allow:/tmp/o,net:10.0.2.2:80", "python3"]),
        argv(&["linux", "--box", "allow:net:a.b:1", "sh"]),
    ] {
        assert!(names_destinations(&yes));
    }
    for no in [
        argv(&["linux", key, "--box", "strict", "python3"]),
        argv(&["linux", key, "--box", "allow:/tmp/o,/home/n", "sh"]),
        argv(&["linux", key, "--box", "offline", "sh", "--box", "allow:net:a.b:1"]),
        argv(&["linux", "sh", "--box", "allow:net:a.b:1"]),
        argv(&["linux", key, key, "--box", "allow:net:a.b:1"]),
        argv(&["--box", "allow:net:a.b:1"]),
        argv(&["linux", "--box"]),
        argv(&["linux", "--box", "net:a.b:1"]),
        Vec::new(),
    ] {
        assert!(!names_destinations(&no), "{:?}", String::from_utf8_lossy(&no));
    }
}

#[test]
fn a_caller_without_network_is_refused_the_network_role() {
    let required = field(LINUX, "CAPSULE_REQUIRED_CAPS");
    let optional = field(LINUX, "CAPSULE_OPTIONAL_CAPS");
    assert_ne!(optional & NET, 0, "the manifest lets a role ask for Network");
    let manifest = [required.to_be_bytes(), optional.to_be_bytes()].concat();
    world(Some(30), &[(30, required)]);
    assert_eq!(check(&manifest, required | NET, b"tool.linux.net"), Err(-1));
    world(Some(31), &[(31, required | NET)]);
    assert_eq!(check(&manifest, required | NET, b"tool.linux.net"), Ok(()));
    world(Some(32), &[(32, field(TERMINAL, "CAPSULE_REQUIRED_CAPS"))]);
    assert_eq!(check(&manifest, required | NET, b"tool.linux.net"), Ok(()), "the Terminal");
}
