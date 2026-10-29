// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Every capsule line ends in its own newline and fits the kernel's 256
//! bytes a call: on 28 October the q35 boot showed `[ACPI] dev` and `prt`
//! lines run together, since the kernel passes capsule bytes through.

use crate::line_frame::framed;

#[test]
fn a_line_ends_in_its_newline() {
    assert_eq!(framed(b"[ACPI] dev x".to_vec(), 256), b"[ACPI] dev x\n".to_vec());
    assert_eq!(framed(Vec::new(), 256), b"\n".to_vec());
}

#[test]
fn a_long_line_is_cut_and_still_ends_in_its_newline() {
    let out = framed(vec![b'a'; 400], 256);
    assert_eq!(out.len(), 256);
    assert_eq!(out.last(), Some(&b'\n'));
    assert!(out[..255].iter().all(|&b| b == b'a'));
}

#[test]
fn the_capsule_sends_every_line_framed() {
    let log = include_str!("../../capsule_acpi/src/sys/log.rs");
    assert!(log.contains("let line = framed(self.0, MAX);"));
    assert_eq!(log.matches("mk_debug(").count(), 1, "a second way out of the line");
}
