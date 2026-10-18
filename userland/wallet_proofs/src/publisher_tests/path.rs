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

use crate::publisher::{attest_sig_path, binding_sig_path, doc_path_ok};

#[test]
fn only_a_plain_path_under_data_is_read() {
    for ok in ["/data/qwen.entry", "/data/pub/qwen-small.entry"] {
        assert!(doc_path_ok(ok), "{ok}");
    }
    let long = format!("/data/{}", "a".repeat(200));
    for no in [
        "/etc/x",
        "/data/",
        "/data/../etc/x",
        "/data/a/./b",
        "data/x",
        "/data//x",
        "/data/a b",
        long.as_str(),
    ] {
        assert!(!doc_path_ok(no), "{no}");
    }
}

#[test]
fn signatures_go_beside_the_document() {
    assert_eq!(binding_sig_path("/data/q.entry").as_deref(), Some("/data/q.entry.binding.sig"));
    assert_eq!(
        attest_sig_path("/data/q.entry", "qwen-small@1").as_deref(),
        Some("/data/q.entry.qwen-small@1.attest.sig")
    );
    assert_eq!(
        attest_sig_path("/data/q.entry", "../x"),
        None,
        "a release id never names a directory"
    );
    assert_eq!(attest_sig_path("/data/q.entry", "a/b"), None);
    assert_eq!(binding_sig_path("/tmp/q.entry"), None);
}
