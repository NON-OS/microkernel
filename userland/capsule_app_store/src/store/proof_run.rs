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

//! When the question shows, and the run a yes starts: against the release
//! the install finished as when this store saw it, else the default ("").

use super::super::progress::Progress;
use super::super::state::State;
use super::proof_debug::say;
use super::proof_offer::offerable;
use super::proof_seq::{ended, Phase};
use super::proof_state::Run;

impl State {
    /// The first time an installed Linux listing is selected, its question;
    /// a run that has ended is let go once its listing is left. True when
    /// the pane changes.
    pub fn offer_proof(&mut self) -> bool {
        let at = self.current().map(|l| (l.id.clone(), l.progress == Progress::Installed));
        let p = &mut self.proofs;
        let here = |id: &[u8]| at.as_ref().is_some_and(|(at, _)| at[..] == *id);
        let left = p.run.as_ref().is_some_and(|r| ended(r.phase) && !here(&r.listing));
        let gone = p.asking.as_deref().is_some_and(|id| !here(id));
        if left {
            p.run = None;
        }
        if gone {
            p.asking = None;
        }
        let moving = p.run.as_ref().is_some_and(|r| !ended(r.phase));
        let at = at.filter(|(id, i)| !moving && offerable(id, *i));
        let Some((id, _)) = at else { return left || gone };
        let asked = p.offered.first(&id);
        if asked {
            p.asking = Some(id);
        }
        left || gone || asked
    }

    /// Yes: the run starts, and the question goes.
    pub(super) fn prove(&mut self, id: &[u8]) -> bool {
        let Some(name) = self.current().map(|l| l.name.clone()) else { return false };
        let p = &mut self.proofs;
        let release = p.installed.iter().find(|(l, _)| l[..] == *id).map(|(_, r)| r.clone());
        p.asking = None;
        p.run = Some(Run {
            listing: id.to_vec(),
            name,
            release: release.unwrap_or_default(),
            policy: [0; 256],
            listing_key: [0; 32],
            phase: Phase::Policy,
            port: 0,
            job: 0,
            late_ms: 0,
            got: None,
            cancelled: false,
        });
        say(&[b"asked: ", id]);
        true
    }
}
