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

//! _OSC committed directly, with no query first: for a caller that asks
//! only what it already implements and has nothing to learn from what
//! firmware would give up. `words[0]` is the status word, sent as 0.

use super::eval::Eval;
use super::osc::call;
use super::osc_status::OscError;
use crate::load::Aml;
use crate::namespace::Id;
use alloc::vec::Vec;

/// Firmware's answer to the commit, as it granted it.
pub fn commit_only(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    osc: Id,
    uuid: (&[u8; 16], u64),
    words: &[u32],
) -> Result<Vec<u32>, OscError> {
    let mut w = words.to_vec();
    if let Some(status) = w.first_mut() {
        *status = 0;
    }
    call(aml, eval, osc, uuid, &w)
}
