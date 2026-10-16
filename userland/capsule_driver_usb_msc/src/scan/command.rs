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

//! The command a unit's asking is at, and none before it is due. INQUIRY
//! until one is answered, TEST UNIT READY until the medium is ready, then
//! READ CAPACITY (`sized`). A unit with no medium is passed once the device
//! has had its second to start a card (`units`), so an empty card reader is
//! given up after two TEST UNIT READYs.

use super::ask::Step;
use super::asking::{Asked, Asking};
use super::identity::Identity;
use super::report::Stage;
use crate::disk::{capacity, inquiry, test_ready, Unit};
use crate::protocol::E_TIMEDOUT;
use crate::state::State;

impl Asking {
    pub(super) fn command(mut self, state: &mut State, served: Option<Identity>) -> Step {
        let disk = self.disk;
        if !self.due.expired() {
            return Step::Again(self);
        } else if self.until.expired() {
            return Step::Failed(disk, Stage::NotReady, E_TIMEDOUT);
        }
        match self.asked {
            Asked::Inquiry => match inquiry(&disk, state) {
                Ok(named) => Self { named, ..self }.then(Asked::Ready { started: false }),
                Err(e) => self.again(e, true),
            },
            Asked::Ready { mut started } => {
                let said = test_ready(&disk, state, &mut started);
                self.asked = Asked::Ready { started };
                match said {
                    Ok(Unit::Ready) => self.then(Asked::Capacity),
                    Ok(Unit::NoMedium) => self.no_medium(served),
                    Ok(Unit::Attention | Unit::NotYet) => self.again(E_TIMEDOUT, false),
                    Err(e) => self.again(e, true),
                }
            }
            Asked::Capacity => match capacity(&disk, state) {
                Ok(size) => self.sized(size, served),
                Err(e) => Step::Failed(disk, Stage::Capacity, e),
            },
        }
    }
}
