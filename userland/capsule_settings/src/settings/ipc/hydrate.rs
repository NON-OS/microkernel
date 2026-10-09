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

use nonos_libc::mk_yield;

use crate::settings::schema::ALL_FIELDS;
use crate::settings::state::{store_value, State, StatusKind};

use super::hydrate_pass::{hydrate_fields, hydrate_final, hydrate_report};
use super::op_get::op_get;

/// Read every stored value into the panel. A pass that could not read them all
/// says so in the status strip: the rows it missed show defaults, which must
/// not pass for the values the policy service holds.
/// Read every stored value into `state`. False when the policy store did not
/// answer and the pass should run again (`HYDRATE_RETRY_MS`).
pub fn hydrate(state: &mut State) -> bool {
    if !state.policy_ready {
        return false;
    }
    let port = state.policy_port;
    let get = |field| {
        let value = op_get(port, field);
        mk_yield();
        value
    };
    let err = hydrate_fields(ALL_FIELDS, get, |field, v| store_value(state, field, v));
    match err {
        Some(err) => state.status.set(StatusKind::Error, hydrate_report(err)),
        // A pass that follows a silent one takes its report down with it.
        None if state.status.as_slice() == hydrate_report(super::IpcError::RecvTimeout) => {
            state.status.set(StatusKind::Idle, b"")
        }
        None => {}
    }
    hydrate_final(err)
}
