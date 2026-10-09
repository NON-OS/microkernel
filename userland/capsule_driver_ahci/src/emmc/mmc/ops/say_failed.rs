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

//! The console line for a failed step: what, why, and the card's status.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{reason, EmmcError};
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::r1::{first_error, state_name};

/// Log `what` failed and why.
pub fn say_failed<M: Mmio, C: Clock, L: Log>(h: &Host<M, C, L>, what: &[u8], e: EmmcError) {
    let mut l = Line::new();
    l.s(what).s(b" failed: ").s(reason(e).as_bytes());
    match e {
        EmmcError::CmdError { err, .. } | EmmcError::DataError { err, .. } => {
            l.s(b" (err ").hex(err as u64).s(b")");
        }
        EmmcError::Status { status, .. } | EmmcError::BadState(status) => {
            l.s(b" (status ").hex(status as u64).s(b", ").s(state_name(status).as_bytes());
            if let Some(name) = first_error(status) {
                l.s(b", ").s(name.as_bytes());
            }
            l.s(b")");
        }
        EmmcError::Broker(r) => {
            l.s(b" (").dec(r.unsigned_abs()).s(b")");
        }
        _ => {}
    }
    h.say(&l);
}
