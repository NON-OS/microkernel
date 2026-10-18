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

//! Step 5: the container carries the release's own policy, the record names
//! its app, and the policy is this listing's or a pinned vendor one.

use super::bind::app_id;
use super::expected::{Expected, PolicyView};
use super::refusal::Refusal;
use crate::record::Record;

pub(super) fn policy(section: &[u8], rec: &Record, ex: &Expected) -> Result<PolicyView, Refusal> {
    if section != ex.policy {
        return Err(Refusal::Policy);
    }
    let view = (ex.read_policy)(ex.policy).ok_or(Refusal::Policy)?;
    let ours = if view.listing_id == [0; 32] {
        ex.pins.vendor_policies.contains(ex.policy)
    } else {
        view.listing_id == ex.listing_id
    };
    if !ours {
        return Err(Refusal::Policy);
    }
    if rec.app_id != app_id(ex.policy, &view.program_blake3) {
        return Err(Refusal::AppId);
    }
    Ok(view)
}
