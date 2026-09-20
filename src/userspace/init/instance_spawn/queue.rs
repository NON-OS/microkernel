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

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

/// An app that declares instance-window endpoints in its signed manifest.
/// The syscall maps the requested handle to one of these before queueing.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PendingApp {
    Terminal,
    Browser,
    TextEditor,
    Settings,
    Calculator,
    Clock,
    About,
    Snake,
    WalletNonos,
    FileManager,
    ProcessManager,
    AudioPlayer,
    VideoPlayer,
}

impl PendingApp {
    /// The capsule name behind this request, for the one place it matters: a
    /// spawn that was refused.
    pub(super) fn name(self) -> &'static [u8] {
        match self {
            PendingApp::Terminal => b"app.terminal",
            PendingApp::Browser => b"app.browser",
            PendingApp::TextEditor => b"app.text_editor",
            PendingApp::Settings => b"app.settings",
            PendingApp::Calculator => b"app.calculator",
            PendingApp::Clock => b"app.clock",
            PendingApp::About => b"app.about",
            PendingApp::Snake => b"app.snake",
            PendingApp::WalletNonos => b"app.nonos_wallet",
            PendingApp::FileManager => b"app.file_manager",
            PendingApp::ProcessManager => b"app.process_manager",
            PendingApp::AudioPlayer => b"app.audio_player",
            PendingApp::VideoPlayer => b"app.video_player",
        }
    }
}

// A click enqueues one request; a few in flight at once is the most a user
// produces, and the cap keeps a stuck drain from growing without bound.
const MAX_PENDING: usize = 8;

pub(super) static PENDING: Mutex<Vec<PendingApp>> = Mutex::new(Vec::new());

/// Record a spawn request. Returns false only when the queue is saturated,
/// which the caller treats as "try again", never as a hard failure.
pub(super) fn push(app: PendingApp) -> bool {
    // Raising init is part of queueing, not a separate courtesy: work left in
    // a queue nobody is scheduled to drain is work that never happens, and the
    // caller was told it was accepted.
    super::super::wake::nudge();
    let mut q = PENDING.lock();
    if q.len() >= MAX_PENDING {
        return false;
    }
    q.push(app);
    super::priority::raise();
    true
}

/// Return init to its idle band once nothing is left to spawn.
pub(super) fn settle() {
    let Some(q) = PENDING.try_lock() else {
        return;
    };
    if q.is_empty() {
        super::priority::restore();
    }
}

/// Take everything queued so far. Called from init's context only.
pub(super) fn take() -> Vec<PendingApp> {
    let mut q = match PENDING.try_lock() {
        Some(q) => q,
        None => return Vec::new(),
    };
    if q.is_empty() {
        return Vec::new();
    }
    core::mem::take(&mut *q)
}

/// Whether any window-instance request is waiting to be drained.
pub(crate) fn has_pending() -> bool {
    match PENDING.try_lock() {
        Some(q) => !q.is_empty(),
        None => true,
    }
}
