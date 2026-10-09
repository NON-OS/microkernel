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


//! What one lookup holds while it runs.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::crypto::Ephemeral;
use crate::onion::desc::IntroPoint;
use crate::onion::lookup::Lookup;
use crate::onion::pow::{PowParams, PowSolution, Puzzle};
use crate::path::Relay;

/// How far a lookup has got.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /// Fetching the descriptor from one responsible HSDir after another.
    Directory,
    /// Building the rendezvous circuit and waiting for it to be established.
    Rendezvous,
    /// Introducing through one introduction point after another.
    Introduce,
    /// Introduced; waiting for the service to meet us at the rendezvous point.
    Waiting,
}

/// Where the puzzle for the next introduction stands.
pub enum PowState {
    /// Nothing started for the next introduction.
    Idle,
    /// Boxed: a puzzle holds its HashX program, about 4 KB, and a job
    /// should not carry that inline when it is not solving.
    Solving(Box<Puzzle>),
    /// Solved and not yet sent.
    Solved(PowSolution),
    /// The next introduction goes out without a solution.
    Skipped,
}

/// How a lookup ended, set by a reply and acted on at the next tick.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    /// The service's hop is up and BEGIN has been sent.
    Connected,
    /// The stream ends with this END reason.
    Failed(u8),
}

pub struct OnionJob {
    /// The caller's stream, in Opening until BEGIN goes out.
    pub stream: u16,
    pub port: u16,
    pub lookup: Lookup,
    pub stage: Stage,
    /// When the whole lookup gives up, and when the current step does.
    pub deadline: u64,
    pub step_deadline: u64,
    /// Responsible HSDirs not yet asked, in the order they will be.
    pub hsdirs: Vec<Relay>,
    /// Introduction points not yet tried.
    pub intro_points: Vec<IntroPoint>,
    pub cookie: [u8; 20],
    pub rendezvous_point: Option<Relay>,
    /// The rendezvous circuit, by id.
    pub rend: Option<u32>,
    /// The HSDir or introduction circuit in flight, by id.
    pub circuit: Option<u32>,
    /// The BEGIN_DIR stream to the HSDir, by id.
    pub dir_stream: Option<u16>,
    /// Whether the current step's request has gone out.
    pub sent: bool,
    /// The ephemeral X25519 key of the introduction in flight, and the
    /// point it was made to. Both are needed again for RENDEZVOUS2.
    pub ephemeral: Option<Ephemeral>,
    pub introduced: Option<IntroPoint>,
    pub outcome: Option<Outcome>,
    /// The introduction points came from the descriptor cache, so when they
    /// all fail the descriptor is fetched again before giving up.
    pub from_cache: bool,
    /// The rendezvous point acknowledged the cookie.
    pub rend_ready: bool,
    /// The puzzle the service's descriptor asks for, if any.
    pub pow: Option<PowParams>,
    pub pow_state: PowState,
    /// Introduction points that did not get us through; each raises the
    /// effort of the next puzzle.
    pub unreachable: u32,
    /// Solving stops at this time; set by the lookup's first puzzle.
    pub solve_until: Option<u64>,
}
