// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Clicking a Launchpad package tile asks the installer to verify the file
//! and, on success, raises the install-consent modal with the attested
//! summary; a rejection is said out loud instead of doing nothing.

use alloc::vec::Vec;

use nonos_app_skeleton::log_line::{say as log, Line};

use crate::render::sync_toast_layer;
use crate::state::says::{named, package};
use crate::state::toast::TOAST_TEXT_MAX;
use crate::state::{Context, NotifyLevel, PkgInstallPrompt};

pub fn begin(ctx: &mut Context, index: usize) {
    let Some(name) = ctx.pkg_files.get(index) else { return };
    let mut path = Vec::with_capacity(b"/pkgs/".len() + name.len());
    path.extend_from_slice(b"/pkgs/");
    path.extend_from_slice(name.as_bytes());
    match crate::installer_client::pkg_query(&path) {
        Ok(summary) => ctx.pending_pkg_install = Some(PkgInstallPrompt { path, summary }),
        Err(code) => report_rejected(ctx, code),
    }
}

/// A refused package must never look like a dead click: the installer's errno
/// is the only thing distinguishing a bad signature from a missing file, and
/// it is said in words (`state::says::package`), not as a number.
pub(crate) fn report_rejected(ctx: &mut Context, code: i32) {
    let mut line = [0u8; TOAST_TEXT_MAX];
    let n = named(b"Package: ", package(code), &mut line);
    ctx.toasts.push(&line[..n], NotifyLevel::Error, crate::server::toast_clock::now());
    sync_toast_layer(ctx);
    let said = Line::new(b"SHELL").text(b"package not installed: ");
    let _ = log(&said.text(package(code)).text(b" (installer ").num(code.into()).text(b")"));
}

pub(crate) fn push_i32(out: &mut Vec<u8>, v: i32) {
    let mut n = v as i64;
    if n < 0 {
        out.push(b'-');
        n = -n;
    }
    let mut digits = [0u8; 10];
    let mut i = digits.len();
    loop {
        i -= 1;
        digits[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    out.extend_from_slice(&digits[i..]);
}
