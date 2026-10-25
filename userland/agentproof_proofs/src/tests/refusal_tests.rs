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

//! Every refusal the format makes, with the line it names and the reason.

use agentproof_policy::format::Error;
use agentproof_policy::{compile, parse};

use super::file;

pub const LINES: &str = "a line is base, write, network, connect or a # comment";
pub const NET: &str = "network is `none`; name each destination on a connect line";

pub fn refused(lines: &[&str]) -> (usize, &'static str) {
    let Error { line, why } = parse(&file(lines)).unwrap_err();
    (line, why)
}

#[test]
fn the_header_comes_first_and_once() {
    assert_eq!(refused(&[]), (0, "the first line is `nonos-box 1`"));
    assert_eq!(refused(&["# only", ""]), (0, "the first line is `nonos-box 1`"));
    assert_eq!(refused(&["base strict"]), (1, "the first line is `nonos-box 1`"));
    assert_eq!(refused(&["nonos-box 2"]), (1, "this reader knows `nonos-box 1` only"));
    assert_eq!(refused(&["nonos-box"]), (1, "this reader knows `nonos-box 1` only"));
    assert_eq!(refused(&["nonos-box 1 1"]), (1, "this reader knows `nonos-box 1` only"));
    assert_eq!(refused(&["nonos-box 1", "nonos-box 1"]), (2, "the header is named once"));
}

#[test]
fn each_line_is_known_and_once() {
    let h = "nonos-box 1";
    assert_eq!(refused(&[h, "base"]), (2, "a line is its name and one word"));
    assert_eq!(refused(&[h, "base strict offline"]), (2, "a line is its name and one word"));
    assert_eq!(refused(&[h, "base all"]), (2, "base is strict, offline or folders"));
    assert_eq!(refused(&[h, "base strict", "base strict"]), (3, "base is named once"));
    assert_eq!(refused(&[h, "network none", "network none"]), (3, "network is named once"));
    assert_eq!(refused(&[h, "read /etc"]), (2, LINES));
    assert_eq!(refused(&[h, "Base strict"]), (2, LINES));
}

#[test]
fn network_is_none_or_absent() {
    let h = "nonos-box 1";
    assert_eq!(refused(&[h, "network any"]), (2, NET));
    assert_eq!(refused(&[h, "network 10.0.0.1:80"]), (2, NET));
}

#[test]
fn a_word_past_255_bytes_is_refused_when_compiled() {
    let long: Vec<String> = (0..16).map(|i| format!("write /var/tmp/{i:0>12}")).collect();
    let mut lines = vec!["nonos-box 1", "base folders", "network none"];
    lines.extend(long.iter().map(String::as_str));
    let p = parse(&file(&lines)).unwrap();
    assert!(compile(&p).is_err_and(|w| w.starts_with("the box word would be over 255 bytes")));
}
