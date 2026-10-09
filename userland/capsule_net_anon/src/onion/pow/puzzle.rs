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


//! One puzzle being solved, a slice at a time.
//!
//! The challenge is "Tor hs intro v1\0" | blinded key | seed | nonce |
//! effort (big endian), 100 bytes. Each nonce gives an Equi-X instance with
//! about two solutions; a solution counts when
//! BLAKE2b-32(challenge | solution), read big endian, times the effort fits
//! in 32 bits. Otherwise the nonce steps as a little-endian counter and the
//! next instance is solved. This is hs_pow_solve's loop, cut into slices.

extern crate alloc;

use alloc::boxed::Box;

use nonos_equix::{Blake2b, HashX, Solution, Solver, MAX_SOLS, SOLUTION_BYTES};

use super::PowParams;

const PREFIX: &[u8; 16] = b"Tor hs intro v1\0";
const CHALLENGE_BYTES: usize = 100;
const NONCE_AT: usize = 80;
const EFFORT_AT: usize = 96;
/// TRUNNEL_EXT_TYPE_POW, and TRUNNEL_POW_VERSION_EQUIX inside it.
const EXT_TYPE_POW: u8 = 0x02;
const POW_VERSION_EQUIX: u8 = 0x01;
/// The extension field's body: version, nonce, effort, seed head, solution.
const POW_BODY: usize = 1 + 16 + 4 + 4 + SOLUTION_BYTES;
/// The whole field as INTRODUCE1 carries it: type, length, body.
pub const EXTENSION_BYTES: usize = 2 + POW_BODY;

/// What INTRODUCE1 carries for a solved puzzle.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PowSolution {
    pub nonce: [u8; 16],
    pub effort: u32,
    /// The seed's first four bytes, so the service knows which of its two
    /// current seeds the solution is for.
    pub seed_head: [u8; 4],
    pub solution: [u8; SOLUTION_BYTES],
}

pub struct Puzzle {
    challenge: [u8; CHALLENGE_BYTES],
    effort: u32,
    seed_head: [u8; 4],
    /// The function for the current nonce, built when its first slice runs.
    hashx: Option<HashX>,
    solver: Box<Solver>,
    /// Equi-X instances finished so far, one per nonce tried.
    pub solves: u32,
}

impl Puzzle {
    /// A puzzle for the service with blinded key `blinded` under `params`, at
    /// `effort`, starting from `nonce`, which the caller draws at random.
    /// `None` when the solver's memory cannot be had.
    pub fn new(blinded: &[u8; 32], params: &PowParams, effort: u32, nonce: [u8; 16]) -> Option<Self> {
        let mut challenge = [0u8; CHALLENGE_BYTES];
        challenge[..16].copy_from_slice(PREFIX);
        challenge[16..48].copy_from_slice(blinded);
        challenge[48..80].copy_from_slice(&params.seed);
        challenge[NONCE_AT..EFFORT_AT].copy_from_slice(&nonce);
        challenge[EFFORT_AT..].copy_from_slice(&effort.to_be_bytes());
        let mut seed_head = [0u8; 4];
        seed_head.copy_from_slice(&params.seed[..4]);
        let solver = Box::new(Solver::new()?);
        Some(Self { challenge, effort, seed_head, hashx: None, solver, solves: 0 })
    }

    pub fn effort(&self) -> u32 {
        self.effort
    }

    /// Hashes up to `budget` more indices of the current instance, and when
    /// that finishes the instance, checks its solutions. A solution that meets
    /// the effort is returned; otherwise the next nonce is set up for the
    /// next call.
    pub fn step(&mut self, budget: u32) -> Option<PowSolution> {
        if self.hashx.is_none() {
            match HashX::new(&self.challenge) {
                Some(hashx) => {
                    self.hashx = Some(hashx);
                    self.solver.start();
                }
                // About one challenge in ten thousand selects a program that
                // fails the uniformity rules. It has no solutions; the
                // reference counts it as a solve and moves on.
                None => {
                    self.next_nonce();
                    return None;
                }
            }
        }
        let hashx = self.hashx.as_ref()?;
        if !self.solver.hash_some(hashx, budget) {
            return None;
        }
        let mut found = [Solution::default(); MAX_SOLS];
        let count = self.solver.finish(&mut found);
        for solution in found.iter().take(count) {
            let bytes = solution.to_bytes();
            if meets(&self.challenge, &bytes, self.effort) {
                let mut nonce = [0u8; 16];
                nonce.copy_from_slice(&self.challenge[NONCE_AT..EFFORT_AT]);
                return Some(PowSolution { nonce, effort: self.effort, seed_head: self.seed_head, solution: bytes });
            }
        }
        self.next_nonce();
        None
    }

    fn next_nonce(&mut self) {
        self.solves = self.solves.saturating_add(1);
        self.hashx = None;
        for byte in self.challenge[NONCE_AT..EFFORT_AT].iter_mut() {
            *byte = byte.wrapping_add(1);
            if *byte != 0 {
                break;
            }
        }
    }
}

/// The effort check. Widened to 64 bits so the product cannot wrap.
pub fn meets(challenge: &[u8], solution: &[u8; SOLUTION_BYTES], effort: u32) -> bool {
    let mut hash = Blake2b::new(4, &[0; 16]);
    hash.update(challenge);
    hash.update(solution);
    let mut r = [0u8; 4];
    hash.finish(&mut r);
    u64::from(u32::from_be_bytes(r)) * u64::from(effort) <= u64::from(u32::MAX)
}

/// The INTRODUCE1 extension field for a solution, as trn_cell_extension_pow
/// encodes it: integers in network order.
pub fn extension(solution: &PowSolution) -> [u8; EXTENSION_BYTES] {
    let mut out = [0u8; EXTENSION_BYTES];
    out[0] = EXT_TYPE_POW;
    out[1] = POW_BODY as u8;
    out[2] = POW_VERSION_EQUIX;
    out[3..19].copy_from_slice(&solution.nonce);
    out[19..23].copy_from_slice(&solution.effort.to_be_bytes());
    out[23..27].copy_from_slice(&solution.seed_head);
    out[27..].copy_from_slice(&solution.solution);
    out
}
