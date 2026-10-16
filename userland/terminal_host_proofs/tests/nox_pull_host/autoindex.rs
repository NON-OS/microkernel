// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Directory listing extraction for recursive pulls.

use crate::recurse;

#[test]
fn autoindex_extracts_files_and_dirs() {
    let html = br#"<ul><li><a href="a.png">a.png</a></li>
<li><a href="sub/">sub/</a></li></ul>"#;
    let e = recurse::parse_autoindex(html);
    assert_eq!(e.len(), 2);
    assert_eq!(e[0].name, b"a.png");
    assert!(!e[0].is_dir);
    assert_eq!(e[1].name, b"sub/");
    assert!(e[1].is_dir);
}

#[test]
fn autoindex_skips_parent_and_absolute() {
    let html = br#"<a href="../">up</a><a href="/root">r</a><a href="ok.txt">ok</a>"#;
    let e = recurse::parse_autoindex(html);
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].name, b"ok.txt");
}

#[test]
fn autoindex_rejects_traversal_names() {
    let html = br#"<a href="a/../../etc">x</a><a href="s/../../y">y</a><a href="..\z">z</a><a href="good.txt">g</a>"#;
    let e = recurse::parse_autoindex(html);
    assert_eq!(e.len(), 1);
    assert_eq!(e[0].name, b"good.txt");
}
