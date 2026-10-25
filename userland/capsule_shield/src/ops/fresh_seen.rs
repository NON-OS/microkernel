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

//! Fresh addresses, answered from what the worker read after the last job.

use std::sync::Arc;

use nox_shield_core::ffi::Wallet;
use shield_wire::Values;

use super::seen::SEEN;
use super::Answer;

/// The addresses mode 1 may hand out, each with its account index.
type Ahead = Vec<(u32, [u8; 20])>;

/// What mode 1 of OP_FRESH_ADDRESS answers from, read after the last job.
pub(crate) fn seen_ahead() -> Option<(Arc<Wallet>, u32, Ahead)> {
    let seen = SEEN.lock().ok()?;
    seen.as_ref().map(|s| (Arc::clone(&s.wallet), s.ahead.0, s.ahead.1.clone()))
}

/// Answered at once from what the worker read after the last job: the
/// store may be held by a proof, and the service loop never waits on it.
pub fn fresh_address() -> Answer {
    let seen = SEEN.lock().map_err(|_| String::from("the shield is busy"))?;
    let s = seen.as_ref().ok_or_else(|| String::from("Open the shield from the wallet first."))?;
    let mut v = Values::new();
    match &s.fresh {
        Ok(Some(a)) => {
            v.put("address", a);
        }
        Ok(None) => {}
        Err(why) => return Err(format!("the shield store could not name a fresh account: {why}")),
    }
    Ok(v)
}
