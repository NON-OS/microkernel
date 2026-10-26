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

//! What a run keeps of its calls' answers: the box op 8 named, judged, and
//! each GET's chunk, taken into its container: the length taken, or 0 for a
//! chunk proof_get.rs refuses, which the step reads as broken. The reply's
//! copy is wiped either way.

use alloc::vec::Vec;

use zeroize::Zeroize;

use super::super::market::{get_linux_policy, next_id};
use super::proof_box::judge;
use super::proof_fail::Fail;
use super::proof_get::Assembly;
use super::proof_seq::Phase;
use super::proof_state::Run;

impl Run {
    /// Op 8 for this run's listing and release, checked before any ask.
    pub fn boxed(&mut self, market: u32) -> Result<(), Fail> {
        let m = get_linux_policy(market, next_id(), &self.listing, &self.release);
        (self.policy, self.listing_key) = judge(m.as_deref())?;
        Ok(())
    }

    pub fn take(&mut self, mut chunk: Vec<u8>) -> usize {
        if let (None, Phase::Fetching { total, .. }) = (&self.got, self.phase) {
            self.got = Assembly::new(total);
        }
        let took = self.got.as_mut().is_some_and(|g| g.take(&chunk));
        let len = if took { chunk.len() } else { 0 };
        chunk.zeroize();
        len
    }

    /// The container, once whole, out of the run.
    pub fn container(&mut self) -> Option<Vec<u8>> {
        let g = self.got.as_mut().filter(|g| g.done())?;
        Some(core::mem::take(&mut g.bytes))
    }
}
