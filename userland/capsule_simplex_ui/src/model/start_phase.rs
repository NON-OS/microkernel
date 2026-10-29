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

//! Where starting the core is (start.rs), and what one tick of it does.

use alloc::string::String;

pub const LOOK_MS: i64 = 20_000;
pub const EVERY_MS: i64 = 250;
/// MkAppLaunch's busy answer: a run is going.
pub const EBUSY: i64 = -16;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Phase {
    /// A launch to ask, with this word if any.
    Launch(Option<String>),
    /// Launched at `since`, `named` with the person's name or not.
    Look { since: i64, named: bool },
    /// The core never listened: its name is asked for.
    Ask,
    /// The endpoint answered.
    Up,
    /// Nothing more to try, and why.
    Stopped(&'static str),
}

/// What a tick does: the launch to ask, then whether to look.
pub struct Step {
    pub launch: Option<Option<String>>,
    pub look: bool,
}
