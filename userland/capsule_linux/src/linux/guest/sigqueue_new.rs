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

//! A process's signal state as it starts: every disposition at its default,
//! nothing pending, no timers, no waits, and SIGCHLD as its exit signal.

use alloc::vec::Vec;

use super::sigqueue::Signals;
use super::sigstate::{SigAction, NSIG};

impl Default for Signals {
    fn default() -> Self {
        Signals {
            actions: [SigAction::default(); NSIG],
            pending: Vec::new(),
            threads: Vec::new(),
            outbox: Vec::new(),
            real: None,
            timers: Vec::new(),
            sigwaits: Vec::new(),
            childwaits: Vec::new(),
            leader_gone: false,
            vfork: None,
            exit_signal: super::sigstate::SIGCHLD,
            clone_kids: Vec::new(),
            kid_groups: Vec::new(),
        }
    }
}
