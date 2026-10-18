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

//! How a name resolves through the service registry, prefix by prefix.

use crate::kill_target::{resolve, NAME_MAX};

/* A registry holding the services a desktop boot shows. */
fn registry(name: &[u8]) -> Option<u64> {
    match name {
        b"app.browser" => Some(61),
        b"tool.qwen" => Some(77),
        b"net.anon" => Some(40),
        b"wm" => Some(12),
        _ => None,
    }
}

fn found(name: &[u8]) -> Option<(Vec<u8>, u64)> {
    let mut buf = [0u8; NAME_MAX];
    resolve(name, &mut buf, registry).map(|(len, pid)| (buf[..len].to_vec(), pid))
}

#[test]
fn kill_browser_reaches_the_browser_app() {
    assert_eq!(found(b"browser"), Some((b"app.browser".to_vec(), 61)));
}

#[test]
fn a_full_name_is_taken_as_typed() {
    assert_eq!(found(b"app.browser"), Some((b"app.browser".to_vec(), 61)));
    assert_eq!(found(b"wm"), Some((b"wm".to_vec(), 12)));
}

#[test]
fn tools_and_network_services_resolve_too() {
    assert_eq!(found(b"qwen"), Some((b"tool.qwen".to_vec(), 77)));
    assert_eq!(found(b"anon"), Some((b"net.anon".to_vec(), 40)));
}

#[test]
fn a_name_nothing_answers_to_finds_nothing() {
    assert_eq!(found(b"nosuch"), None);
}

#[test]
fn a_name_too_long_with_its_prefix_is_skipped_not_cut() {
    let long = [b'x'; NAME_MAX - 2];
    let mut buf = [0u8; NAME_MAX];
    let mut tried = Vec::new();
    resolve(&long, &mut buf, |n| {
        tried.push(n.len());
        None
    });
    assert_eq!(tried, vec![NAME_MAX - 2]);
}
