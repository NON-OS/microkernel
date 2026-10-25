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

//! A signing row pressed: the binding, or release i's attest once reviewed,
//! signed and written beside the entry document.

use alloc::format;

use super::rows::{ROW_ATTEST, ROW_BINDING};
use crate::wallet::chain;
use crate::wallet::publish::{sign_attest, sign_binding};
use crate::wallet::state::State;

pub(super) fn sign(state: &mut State, row: u8) {
    let Some(e) = state.publisher.entry.clone() else { return };
    let path = state.publisher.path.clone();
    let chain = chain::current().id;
    let done = match row {
        ROW_BINDING => sign_binding(state, &path, &e, chain),
        r => sign_attest(state, &path, &e, (r - ROW_ATTEST) as usize, chain),
    };
    match done {
        Ok(at) => state.publisher.news = Some(format!("written to {at}")),
        Err(why) => state.failure = Some(why),
    }
}
