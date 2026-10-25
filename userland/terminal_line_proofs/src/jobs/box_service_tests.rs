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

//! The service a typed line runs: `tool.linux.net` only for a `linux --box`
//! whose box names a destination, `tool.<handle>` for every other line.

use super::tool_service::service;

fn of(line: &str) -> String {
    let words: Vec<&[u8]> = line.split(' ').map(str::as_bytes).collect();
    String::from_utf8(service(words[0], &words)).unwrap()
}

#[test]
fn a_box_naming_a_destination_runs_in_the_network_role() {
    assert_eq!(of("linux --box allow:/tmp/o,net:10.0.2.2:80 python3"), "tool.linux.net");
    assert_eq!(of("linux --box allow:net:a.b:1 sh"), "tool.linux.net");
}

#[test]
fn every_other_line_runs_as_named() {
    for line in [
        "linux --box strict python3",
        "linux --box offline sh",
        "linux --box allow:/tmp/o,/home/n sh",
        "linux sh --box allow:net:a.b:1",
        "linux python3",
        "linux --box",
        "linux --box net:a.b:1 sh",
    ] {
        assert_eq!(of(line), "tool.linux", "{line}");
    }
    assert_eq!(of("tokei --box allow:net:a.b:1"), "tool.tokei");
}

/// The glue runs the service this names, and nothing else names one.
#[test]
fn the_spawn_runs_the_service_chosen_here() {
    let spawn = include_str!("../../../capsule_terminal/src/command/builtin/tool_spawn.rs");
    assert!(spawn.contains("let service = super::tool_service::service(handle, args);"));
    assert_eq!(spawn.matches("mk_tool_run(&service, &argv)").count(), 2);
    assert!(!spawn.contains("b\"tool.\""));
}
