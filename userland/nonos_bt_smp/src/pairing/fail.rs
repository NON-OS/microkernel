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

//! Ending pairing: with a Pairing Failed PDU to the peer (Vol 3, Part H,
//! 3.5.5), or silently once the SMP timer has run out (3.4).

use super::fields::pdu;
use super::initiator::Initiator;
use super::state::State;
use super::step::{Outcome, Step};
use crate::pdu::{PAIRING_FAILED, UNSPECIFIED_REASON};

impl Initiator {
    pub(super) fn fail(&mut self, reason: u8) -> Step {
        self.state = State::Failed;
        let mut step = Step::send(pdu(PAIRING_FAILED, &[reason]));
        step.outcome = Some(Outcome::Failed { reason, by_peer: false });
        step
    }

    pub(super) fn time_out(&mut self) -> Step {
        self.state = State::Failed;
        Step::outcome(Outcome::Failed { reason: UNSPECIFIED_REASON, by_peer: false })
    }
}
