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

//! The attest presses: a row opens its review and keeps the digest shown;
//! Sign signs only while the attest still has that digest, so a changed
//! network or entry is reviewed again rather than signed unseen.

use nonos_app_skeleton::EventOutcome;
use nonos_nox_config::{address, Contract};

use super::rows::ROW_ATTEST;
use crate::wallet::chain;
use crate::wallet::publish::{why, Kec};
use crate::wallet::publisher::{attest_act, attest_review, may_sign, AttestAct, AttestReview};
use crate::wallet::screen::hits::Press;
use crate::wallet::state::State;

const CHANGED: &str = "the attest changed since it was shown; review it again";

/// An attest press, done; None for a press that is not one.
pub(super) fn attest(
    state: &mut State,
    press: Press,
    sign: fn(&mut State, u8),
) -> Option<EventOutcome> {
    let asked = state.publisher.attest_ask;
    match attest_act(asked, press) {
        AttestAct::Open(i) => {
            state.publisher.attest_ask = Some(i);
            state.publisher.attest_digest = review_of(state, i).ok().map(|r| r.digest);
        }
        AttestAct::Sign(i) => {
            let shown = state.publisher.attest_digest;
            let now = review_of(state, i).ok().map(|r| r.digest);
            (state.publisher.attest_ask, state.publisher.attest_digest) = (None, None);
            match may_sign(shown, now) {
                true => sign(state, ROW_ATTEST + i as u8),
                false => state.failure = Some(CHANGED),
            }
        }
        AttestAct::Cancel => {
            (state.publisher.attest_ask, state.publisher.attest_digest) = (None, None)
        }
        AttestAct::Other if asked.is_none() => return None,
        AttestAct::Other if press == Press::Dismiss => state.failure = None,
        AttestAct::Other => return Some(EventOutcome::Idle),
    }
    Some(EventOutcome::Repaint)
}

/// Release `i`'s attest as this account would sign it now, or why not.
pub(super) fn review_of(state: &State, i: usize) -> Result<AttestReview, &'static str> {
    let e = state.publisher.entry.as_ref().ok_or("no entry document is open")?;
    let id = chain::current().id;
    let bond =
        address(id, Contract::PublisherBond).map_err(|_| "no Bond contract on this network yet")?;
    attest_review(&Kec, e, i, id, &state.acct.address, &bond).map_err(why)
}
