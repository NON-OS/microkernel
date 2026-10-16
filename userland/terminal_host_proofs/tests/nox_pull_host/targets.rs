// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Target parsing and the default destination it implies.

use crate::support::ok_val;
use crate::target;

#[test]
fn target_parse_ip_with_port() {
    let t = ok_val(target::parse_target(b"10.0.2.2:8000/report.pdf"));
    assert_eq!(t.hostname, b"10.0.2.2");
    assert_eq!(t.host, b"10.0.2.2:8000");
    assert_eq!(t.port, 8000);
    assert_eq!(t.path, b"/report.pdf");
    assert!(!t.is_dir);
}

#[test]
fn target_parse_hostname_default_port() {
    let t = ok_val(target::parse_target(b"example.com/a.txt"));
    assert_eq!(t.hostname, b"example.com");
    assert_eq!(t.host, b"example.com");
    assert_eq!(t.port, 80);
    assert_eq!(t.path, b"/a.txt");
}

#[test]
fn target_parse_dir_trailing_slash() {
    let t = ok_val(target::parse_target(b"host:9/photos/"));
    assert!(t.is_dir);
    assert_eq!(t.port, 9);
}

#[test]
fn target_reject_missing_path_and_bad_port() {
    assert!(target::parse_target(b"host:80").is_err());
    assert!(target::parse_target(b"host:99999/x").is_err());
    assert!(target::parse_target(b"host:/x").is_err());
}

#[test]
fn default_dest_file_and_dir() {
    let f = ok_val(target::parse_target(b"h/sub/report.pdf"));
    assert_eq!(target::default_dest(&f), b"report.pdf");
    let d = ok_val(target::parse_target(b"h/photos/"));
    assert_eq!(target::default_dest(&d), b"photos/");
}

#[test]
fn default_dest_host_root_uses_hostname() {
    let d = ok_val(target::parse_target(b"example.com/"));
    assert_eq!(target::default_dest(&d), b"example.com/");
    let f = ok_val(target::parse_target(b"10.0.2.2:8000/"));
    assert_eq!(target::default_dest(&f), b"10.0.2.2/");
}
