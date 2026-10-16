// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Redirect resolution and its refusals.

use crate::redirect;

#[test]
fn redirect_absolute_path_replaces_whole_path() {
    assert_eq!(redirect::resolve(b"h", b"/a/b", b" /new").unwrap(), b"/new".to_vec());
}

#[test]
fn redirect_relative_resolves_against_cur_dir() {
    assert_eq!(
        redirect::resolve(b"h", b"/dir/file.txt", b"next.txt").unwrap(),
        b"/dir/next.txt".to_vec()
    );
}

#[test]
fn redirect_absolute_url_same_host_extracts_path() {
    assert_eq!(redirect::resolve(b"host", b"/a", b"http://host/x").unwrap(), b"/x".to_vec());
    assert_eq!(redirect::resolve(b"host", b"/a", b"http://host:8080/x").unwrap(), b"/x".to_vec());
    assert_eq!(redirect::resolve(b"host", b"/a", b"HTTP://host/x").unwrap(), b"/x".to_vec());
}

#[test]
fn redirect_absolute_url_no_path_is_root() {
    assert_eq!(redirect::resolve(b"host", b"/a", b"http://host").unwrap(), b"/".to_vec());
}

#[test]
fn redirect_refuses_cross_host() {
    assert!(redirect::resolve(b"host", b"/a", b"http://other/x").is_err());
}

#[test]
fn redirect_refuses_https_downgrade_guard() {
    assert!(redirect::resolve(b"host", b"/a", b"https://host/x").is_err());
}

#[test]
fn redirect_rejects_empty_location() {
    assert!(redirect::resolve(b"host", b"/a", b"   ").is_err());
}
