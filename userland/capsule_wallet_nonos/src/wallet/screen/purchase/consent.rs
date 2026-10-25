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

//! The escrow consent as this wallet says it: terms::lines on the
//! market's record, by the wallet's own name, its payee checksummed with
//! keccak from the crypto service.

use crate::wallet::manifest::manifest;
use crate::wallet::order::intake::{Pending, Step};
use crate::wallet::order::read::Named;
use crate::wallet::order::terms::lines::{lines as said, Lines};
use crate::wallet::publish::Kec;

/// The consent's words, while an escrow purchase waits on it.
pub fn lines(p: &Pending) -> Option<Lines> {
    let (Step::Consent, Ok(Named::Escrow(b)), Some(r)) = (&p.step, &p.read, &p.record) else {
        return None;
    };
    let wallet = core::str::from_utf8(manifest().title).unwrap_or("This wallet");
    Some(said(&Kec, wallet, &p.names, r, b, &p.fees))
}
