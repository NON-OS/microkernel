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

//! Handing a proved spend to a lander, and what happened.

use nox_shield_core::ffi::Wallet;
use shield_wire::Values;

/// After a spend is proved: hand it to a lander, and say what happened. A
/// lander that refused it, or none reached, is said with its reason, and
/// the owner may settle it from the public account.
pub(super) fn publish(v: &mut Values, w: &Wallet) {
    match w.publish_spend() {
        Ok(p) => {
            v.put("published", "1").put("times", &p.times.to_string());
            if let Some(id) = &p.id {
                v.put("handoff", id);
            }
            if let Some(why) = &p.refusal {
                v.put("lander_refusal", why);
            }
        }
        Err(e) => {
            v.put("published", "0").put("lander_refusal", &e.to_string());
        }
    }
}
