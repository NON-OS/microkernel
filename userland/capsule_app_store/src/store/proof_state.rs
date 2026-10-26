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

//! A proof of how an installed Linux app ran (I-B9a): which listings were
//! offered one, the one run asked for, and the container it made, held here
//! in memory and never sent. Sending is a second question, I-B9b's.

use alloc::vec::Vec;

use zeroize::Zeroize;

use super::proof_get::Assembly;
use super::proof_held::Held;
use super::proof_offer::Offered;
use super::proof_seq::Phase;

#[derive(Default)]
pub struct Proofs {
    pub offered: Offered,
    /// The listing whose question the pane shows.
    pub asking: Option<Vec<u8>>,
    pub run: Option<Run>,
    pub held: Option<Held>,
    /// The release id each listing's install finished as, when this store
    /// saw it finish (Progress::Installed), so a proof is asked of that box.
    pub installed: Vec<(Vec<u8>, Vec<u8>)>,
}

pub struct Run {
    pub listing: Vec<u8>,
    pub name: Vec<u8>,
    /// The installed release's id, or empty for the default when unknown.
    pub release: Vec<u8>,
    pub policy: [u8; 256],
    pub listing_key: [u8; 32],
    pub phase: Phase,
    pub port: u32,
    pub job: u32,
    /// When a Late comes: the prover's start, or the job's quiet spell.
    pub late_ms: i64,
    pub got: Option<Assembly>,
    /// `n` while it ran: it goes on quietly, and what it makes is wiped.
    pub cancelled: bool,
}

impl Drop for Run {
    fn drop(&mut self) {
        if let Some(g) = self.got.as_mut() {
            g.bytes.zeroize();
        }
    }
}
