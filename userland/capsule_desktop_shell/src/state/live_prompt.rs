// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Whether to offer the installer at the start of a live session. Setup sets
//! Persistent off for an amnesic session and the policy store sets it back on
//! when it restores an installed one, before the desktop starts. A live
//! session is the one where nothing is kept, so it is the one a person must be
//! told about, once, with the way to keep things.

/// The prompt's place in this session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LivePrompt {
    /// The policy store has not answered yet.
    Asking(u8),
    /// A live session: the prompt is up.
    Showing,
    /// Answered, dismissed, or an installed session: never again this boot.
    Done,
}

/// Seconds to keep asking a policy store that does not answer. With no
/// answer there is no telling, so nothing is shown.
pub const ASK_TICKS: u8 = 10;

impl Default for LivePrompt {
    fn default() -> Self {
        Self::new()
    }
}

impl LivePrompt {
    pub const fn new() -> Self {
        LivePrompt::Asking(0)
    }

    /// The next state, given what the policy store said of Persistent this
    /// tick: Some(false) is a live session, Some(true) an installed one.
    pub fn step(self, persistent: Option<bool>) -> Self {
        match (self, persistent) {
            (LivePrompt::Asking(_), Some(false)) => LivePrompt::Showing,
            (LivePrompt::Asking(_), Some(true)) => LivePrompt::Done,
            (LivePrompt::Asking(n), None) if n + 1 >= ASK_TICKS => LivePrompt::Done,
            (LivePrompt::Asking(n), None) => LivePrompt::Asking(n + 1),
            (other, _) => other,
        }
    }

    pub fn showing(self) -> bool {
        self == LivePrompt::Showing
    }
}
