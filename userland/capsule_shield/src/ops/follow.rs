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

//! Following the spend proved last, and each one kept before it.

use nox_shield_core::ffi::Wallet;
use shield_wire::Values;

use super::followed::followed;
use super::held::wallet;
use super::publish::publish;
use super::sync::scan_failed;
use super::Answer;

fn follow_earlier(v: &mut Values, w: &Wallet) {
    match w.follow_earlier() {
        Ok(kept) => {
            for k in &kept {
                crate::reply::earlier(v, &k.id, &followed!(k.follow));
            }
        }
        Err(e) => {
            v.put("earlier_why", &e.to_string());
        }
    }
}

pub fn follow() -> Answer {
    let w = wallet()?;
    let mut v = Values::new();
    follow_earlier(&mut v, &w);
    let s = match w.follow_spend() {
        Ok(s) => s,
        // No record: nothing was published yet, so there is nothing to
        // follow. Try a lander again; until one takes it, the owner may
        // settle it. Any other error (the network, a locked store) is said
        // as it is, and the spend is followed again on the next ask.
        Err(nox_shield_core::error::WalletError::Unavailable) => {
            publish(&mut v, &w);
            let taken = shield_wire::field(v.text(), "published") == Some("1")
                && shield_wire::field(v.text(), "lander_refusal").is_none();
            v.put("state", if taken { "republished" } else { "unpublished" })
                .put("self_settle", if taken { "0" } else { "1" });
            return Ok(v);
        }
        Err(e) => return Err(scan_failed(e)),
    };
    crate::reply::follow(&mut v, &followed!(s));
    Ok(v)
}
