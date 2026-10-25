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

//! `connect` lines: the destinations a box reaches, compiled into the `net:`
//! items of its word and run in the network role; every way to misname one
//! is refused on its line.

use agentproof_policy::boxed::preset::parse as word_parse;
use agentproof_policy::{compile, parse};

use super::file;
use super::refusal_tests::refused;

const H: &str = "nonos-box 1";
const COMMA: &str = "a connect line names one destination, with no comma";

#[test]
fn destinations_become_net_items_of_the_box() {
    let lines =
        [H, "base folders", "write /tmp/out", "connect API.example.com:443", "connect 10.0.2.2:80"];
    let c = compile(&parse(&file(&lines)).unwrap()).unwrap();
    assert_eq!(c.word, b"allow:/tmp/out,net:API.example.com:443,net:10.0.2.2:80");
    assert_eq!(c.policy, word_parse(&c.word).unwrap());
    assert!(!c.policy.dests.is_empty());
    let twice = [H, "base folders", "write /tmp/out", "connect 10.0.2.2:80", "connect a.b:1"];
    let once = [H, "base folders", "write /tmp/out", "connect a.b:1", "connect 10.0.2.2:80"];
    let hash = |l: &[&str]| compile(&parse(&file(l)).unwrap()).unwrap().hash;
    assert_eq!(hash(&twice), hash(&once), "the order of connect lines is not the box");
}

#[test]
fn a_destination_is_one_canonical_item() {
    let b = [H, "base folders", "write /tmp/out"];
    let at = |l: &str| refused(&[&b[..], &[l]].concat());
    assert_eq!(at("connect a.b:1,/home"), (4, COMMA));
    assert_eq!(at("connect a.b:1,net:c.d:2").1, COMMA);
    assert_eq!(at("connect a.b").1, "a destination names its port");
    assert_eq!(at("connect a.b:0").1, "a port is 1 to 65535");
    assert_eq!(at("connect a.b:65536").1, "a port is 1 to 65535");
    let dup = [&b[..], &["connect A.b:1", "connect a.B:1"]].concat();
    assert_eq!(refused(&dup), (5, "a destination is named once"));
}

#[test]
fn connect_goes_with_folders_and_not_with_network_none() {
    let both = [H, "base folders", "write /tmp/o", "network none", "connect a.b:1"];
    assert_eq!(refused(&both), (5, "connect lines replace `network none`"));
    for base in ["base strict", "base offline"] {
        assert_eq!(refused(&[H, base, "connect a.b:1"]), (3, "connect lines go with base folders"));
    }
    let no_write = refused(&[H, "base folders", "connect a.b:1"]);
    assert_eq!(no_write, (0, "base folders needs at least one write line"));
}

#[test]
fn at_most_64_destinations() {
    let lines: Vec<String> = (1..=65).map(|p| format!("connect h.example:{p}")).collect();
    let mut file_lines = vec![H, "base folders", "write /tmp/o"];
    file_lines.extend(lines.iter().map(String::as_str));
    assert_eq!(refused(&file_lines), (68, "a file has at most 64 connect lines"));
}
