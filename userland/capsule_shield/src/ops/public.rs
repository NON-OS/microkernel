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

//! What the public account sends: a reviewed deposit, and pending deposits taken back.

use shield_wire::Values;

use super::held::wallet;
use super::{refuse, Answer};

pub fn confirm(f: &[&str]) -> Answer {
    let [id] = f else { return refuse("Name the review.") };
    let id: u64 = id.parse().map_err(|_| String::from("That review is not a number."))?;
    let sent = wallet()?.confirm_public_send(id).map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("tx", &sent.hash).put("link", &sent.link);
    Ok(v)
}

pub fn take_back() -> Answer {
    let n = wallet()?.take_back_pending().map_err(|e| e.to_string())?;
    let mut v = Values::new();
    v.put("returned", &n.to_string());
    Ok(v)
}
