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

/*
 * What the person reads before a publisher transaction is signed: the
 * action decoded from the values the wallet itself will encode, as
 * (label, value) rows. Pure: NOX amounts come through `nox`.
 */

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::call::Publisher;
use super::review_bond::{bond_rows, Rows};
use super::text::id;

pub fn review_rows(p: &Publisher, nox: fn(u128) -> String) -> Rows {
    use Publisher::*;
    let s = |t: &str| t.to_string();
    match p {
        Publish { release_id, release_hash, bond_id, uri, .. } => Vec::from([
            ("action", s("publish a release")),
            ("release", id(release_id)),
            ("release hash", id(release_hash)),
            ("bond", id(bond_id)),
            ("fetched from", String::from_utf8_lossy(uri).into_owned()),
        ]),
        WithdrawRelease { listing_id, release_id } => Vec::from([
            ("action", s("withdraw a release")),
            ("listing", id(listing_id)),
            ("release", id(release_id)),
        ]),
        Anchor { anchor, kind, listing_id } => Vec::from([
            ("action", s("anchor a proof on chain")),
            ("anchor", id(anchor)),
            ("proof kind", format!("{kind}")),
            ("listing", id(listing_id)),
        ]),
        _ => bond_rows(p, nox).unwrap_or_default(),
    }
}
