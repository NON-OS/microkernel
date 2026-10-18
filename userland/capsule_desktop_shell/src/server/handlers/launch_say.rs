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

//! What a store app's launch says on screen: a toast for a refused load,
//! naming a failed verification apart from any other refusal, and the
//! words around an app's name.

use crate::render::sync_toast_layer;
use crate::state::{Context, NotifyLevel};
use alloc::vec::Vec;
use nonos_app_skeleton::log_line::{say as log, Line};

const ERR_REJECTED: i32 = -13;

/// A refused load used to look exactly like nothing happening; say why on
/// screen, and call out a verification rejection distinctly since it means
/// the store artifacts themselves failed attestation.
pub(super) fn report_failure(ctx: &mut Context, name: &[u8], status: i32) {
    let text: &[u8] = if status == ERR_REJECTED {
        b"app rejected: failed verification"
    } else {
        b"app failed to launch"
    };
    toast(ctx, text, NotifyLevel::Error);
    let line = Line::new(b"LAUNCH").text(name).text(b": ").text(text);
    let _ = log(&line.text(b" (installer ").num(status.into()).text(b")"));
}

pub(super) fn toast(ctx: &mut Context, text: &[u8], level: NotifyLevel) {
    ctx.toasts.push(text, level, crate::server::toast_clock::now());
    sync_toast_layer(ctx);
}

/// A toast's words around an app's name.
pub(super) fn said(before: &[u8], name: &[u8], after: &[u8]) -> Vec<u8> {
    let mut text = Vec::with_capacity(before.len() + name.len() + after.len());
    text.extend_from_slice(before);
    text.extend_from_slice(name);
    text.extend_from_slice(after);
    text
}

/// The kernel refuses a second spawn on a service endpoint a live instance
/// still holds, and the installer forwards that as this errno. It means the
/// app is already running, not that the launch failed, so it must raise the
/// running window instead of the failure toast.
pub(super) const ERR_EXIST: i32 = -17;
