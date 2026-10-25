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

//! A learned file by example: the narrowest folders, `base strict` for a run
//! that did nothing, and a word too long for a box said so in the file.

use agentproof_policy::{compile, parse};

use super::learn_tests::{learned, lines};

#[test]
fn each_folder_is_the_narrowest_that_lets_the_run_write() {
    let wrote = ["/tmp/out", "/tmp/out/b", "/tmp/out/c/d", "/home/work/x/y.txt", "/tmp/a"];
    let t = learned(&wrote, &[], "");
    let want = ["nonos-box 1", "base folders", "write /home/work/x", "write /tmp/a"];
    assert_eq!(lines(&t), [&want[..], &["write /tmp/out", "network none"]].concat());
    assert_eq!(lines(&learned(&[], &[], "")), ["nonos-box 1", "base strict", "network none"]);
    let net = learned(&[], &["api.example.com:443"], "exec 1");
    let want = ["nonos-box 1", "base folders", "write /tmp", "connect api.example.com:443"];
    assert_eq!(lines(&net), want);
    assert!(net.contains("\n# also refused, which no file allows: exec 1\n"));
}

#[test]
fn a_word_too_long_for_a_box_is_said_in_the_file() {
    let many: Vec<String> = (0..40).map(|i| format!("host{i}.example.com:443")).collect();
    let d: Vec<&str> = many.iter().map(String::as_str).collect();
    let text = learned(&["/tmp/out/a"], &d, "");
    assert!(text.contains("\n# its box word is over 255 bytes: name fewer destinations\n"));
    assert!(compile(&parse(text.as_bytes()).unwrap()).is_err(), "and the reader says so too");
    let few = learned(&["/tmp/out/a"], &d[..3], "");
    assert!(!few.contains("over 255") && compile(&parse(few.as_bytes()).unwrap()).is_ok());
}
