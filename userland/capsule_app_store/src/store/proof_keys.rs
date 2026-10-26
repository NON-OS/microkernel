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

//! The proof's keys, for the selected listing: y proves it, n says not now
//! or stops a run, p offers it again. Enter stays Open for an installed one.

use super::super::progress::Progress;
use super::super::state::State;
use super::proof_debug::say;
use super::proof_offer::offerable;
use super::proof_seq::ended;

const KEY_Y: u32 = b'y' as u32;
const KEY_N: u32 = b'n' as u32;
const KEY_P: u32 = b'p' as u32;

impl State {
    /// Some when `code` is a proof's key: whether anything changed.
    pub fn proof_key(&mut self, code: u32) -> Option<bool> {
        let id = self.current()?.id.clone();
        let asking = self.proofs.asking.as_deref() == Some(&id[..]);
        match code {
            KEY_Y if asking => Some(self.prove(&id)),
            KEY_N if asking => {
                self.proofs.asking = None;
                say(&[b"declined: ", &id]);
                Some(true)
            }
            KEY_N => {
                let live = |r: &&mut super::proof_state::Run| r.listing == id && !ended(r.phase);
                let run = self.proofs.run.as_mut().filter(live).filter(|r| !r.cancelled)?;
                run.cancelled = true;
                say(&[b"stopped: ", &id]);
                Some(true)
            }
            KEY_P => Some(self.offer_again(&id)),
            KEY_Y => Some(false),
            _ => None,
        }
    }

    /// The question again, for an installed Linux listing while no run moves.
    fn offer_again(&mut self, id: &[u8]) -> bool {
        let installed = self.current().is_some_and(|l| l.progress == Progress::Installed);
        let moving = self.proofs.run.as_ref().is_some_and(|r| !ended(r.phase));
        if moving || !offerable(id, installed) {
            return false;
        }
        self.proofs.offered.first(id);
        self.proofs.asking = Some(id.to_vec());
        true
    }
}
