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

//! `kill` takes a name as well as a pid. A name resolves through the service
//! registry as typed, then as app., tool. and net., the way pkill finds a
//! process by name. Before this `kill browser` answered "pid must be a
//! number".

use crate::kill_target::{resolve, target, Target, NAME_MAX};

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
fn a_number_is_a_pid() {
    assert_eq!(target(b"61"), Target::Pid(61));
    assert_eq!(target(b"0"), Target::Pid(0));
}

#[test]
fn a_word_is_a_name() {
    assert_eq!(target(b"browser"), Target::Name);
    assert_eq!(target(b"app.browser"), Target::Name);
    assert_eq!(target(b"dotenv-linter"), Target::Name);
}

#[test]
fn text_no_service_could_carry_is_refused() {
    assert_eq!(target(b""), Target::Unreadable);
    assert_eq!(target(b"a b"), Target::Unreadable);
    assert_eq!(target(b"../wm"), Target::Unreadable);
    assert_eq!(target(&[b'a'; NAME_MAX + 1]), Target::Unreadable);
    assert_eq!(target(b"99999999999999999999999"), Target::Unreadable);
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

#[test]
fn a_refusal_says_why_in_words() {
    use crate::kill_target::refused;
    let perm = String::from_utf8_lossy(refused(-1));
    assert!(perm.contains("Process Manager"), "{perm}");
    assert!(perm.contains("only what it started"), "{perm}");
    assert!(String::from_utf8_lossy(refused(-22)).contains("2, 9 or 15"));
}
