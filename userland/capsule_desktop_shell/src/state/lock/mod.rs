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

//! The lock screen and the prompts that choose its passphrase, as state
//! with no I/O: what each key does, what each answer from the keyring does,
//! and what the shell knows of the verifier. The server drives these
//! (server/lock/), the renderer draws them (render/lock/).

pub mod choose;
pub mod field;
pub mod geometry;
pub mod keys;
pub mod released;
pub mod screen;
pub mod verifier;
pub mod words;

pub use choose::{Choose, ChooseStep, Chosen, Purpose};
pub use screen::{Asked, Checked, LockScreen, Note, Phase};
pub use verifier::{Known, Verifier};

/// Everything the shell holds for the lock.
#[derive(Default)]
pub struct Lock {
    pub screen: LockScreen,
    pub verifier: Verifier,
    /// The prompt that chooses or changes the passphrase, while it is up.
    pub choose: Option<Choose>,
    /// The outputs on the canvas, each with the lock's clock and field.
    pub outputs: alloc::vec::Vec<(u32, u32, u32, u32)>,
    /// When the router was last told the lock (uptime ms); it is told
    /// again each second while locked, so a restarted router hears it.
    pub told_ms: i64,
    /// When the lock screen or the prompt was last let go, as an event's post time:
    /// router input posted at or before it is the lock's (released.rs).
    pub released_ns: Option<u64>,
    /// When the keyring's standing was last asked (uptime ms).
    pub standing_ms: i64,
}

impl Lock {
    /// Whether the chrome must be in the trusted band and hold every input
    /// for the lock: the lock screen, or the prompt.
    pub fn in_band(&self) -> bool {
        self.screen.covers() || self.choose.is_some()
    }

    /// Whether locking is offered: only with a passphrase set.
    pub fn offered(&self) -> bool {
        self.verifier.known() == Known::Set
    }
}
