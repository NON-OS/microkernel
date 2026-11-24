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

//! The service a typed line runs: `tool.<handle>` for every line, a
//! `linux --box` naming a destination included. No line asks for a network
//! role: that run's family is granted the network by its front.

use super::tool_service::service;

fn of(line: &str) -> String {
    let words: Vec<&[u8]> = line.split(' ').map(str::as_bytes).collect();
    String::from_utf8(service(words[0])).unwrap()
}

#[test]
fn every_line_runs_as_named() {
    for line in [
        "linux --box allow:/tmp/o,net:10.0.2.2:80 python3",
        "linux --box allow:net:a.b:1 sh",
        "linux --box strict python3",
        "linux --box offline sh",
        "linux sh --box allow:net:a.b:1",
        "linux python3",
        "linux --box",
    ] {
        assert_eq!(of(line), "tool.linux", "{line}");
    }
    assert_eq!(of("tokei --box allow:net:a.b:1"), "tool.tokei");
}

/// The glue runs the service this names, and nothing else names one.
#[test]
fn the_spawn_runs_the_service_chosen_here() {
    let spawn = include_str!("../../../capsule_terminal/src/command/builtin/tool_spawn.rs");
    assert!(spawn.contains("let service = super::tool_service::service(handle);"));
    assert_eq!(spawn.matches("mk_tool_run(&service, &argv)").count(), 2);
    assert!(!spawn.contains("b\"tool.\"") && !spawn.contains("net_ask"));
}
