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

//! What a search ends with: what it covered, or the first rule broken.

use super::events::{Actor, Event};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Broken {
    /// A wait read DRAWS at 0 while a draw was past its second check with
    /// its decrement, or its stores' drain, still to come.
    QuietMidDraw,
    /// A draw past its second check while a takeover, or bootfb's hold,
    /// believed the console quiet.
    DrawWhileQuiet,
    /// A draw past its second check, or its stores in flight, with GONE set.
    DrawAfterGone,
    /// A draw past its second check, or its stores in flight, while a
    /// takeover's claim is ready: the GPU's driver owns the scanout.
    ReadyMidDraw,
    /// Every takeover that ran ended refused and nothing holds, yet a draw
    /// started alone is stopped: the console never comes back.
    NeverBack,
    /// A takeover's claim was made ready, yet a draw started alone writes.
    BackAfterReady,
    /// `stopped()` read alone is not HELD or GONE or TAKERS above 0.
    StoppedWrong,
    /// Nothing can step, and some actor has not finished.
    Stuck,
}

#[derive(Debug)]
pub struct Covered {
    pub states: usize,
    pub steps: usize,
    pub runs: u128,
}

/// The first rule broken, and the steps that led to it.
#[derive(Debug)]
pub struct Failure {
    pub broken: Broken,
    pub trace: Vec<(Actor, Event)>,
}
