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

//! The background scan the serving loop runs between requests: one passive
//! firmware scan at a time, pumped a pass of received buffers per call, so a
//! scan request is answered at once from what was heard instead of blocking
//! the loop for a whole sweep. Every received management frame goes to the
//! caller as it is drained. The completion carrying this scan's uid ends the
//! sweep; a sweep that runs past its channel budget is given up as stalled,
//! and a firmware or hardware error cause ends it as failed, so a firmware
//! that never completes cannot hold the loop. The caller passes the time, so
//! the proofs drive it along any timeline. A join stops a sweep early
//! (`abort`): the firmware then completes it with the aborted status, which
//! ends it as any completion does.

mod abort;
mod kinds;
mod pump;
mod start;
mod state;

pub use kinds::*;
pub use state::Sweep;
