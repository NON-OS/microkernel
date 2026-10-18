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
 * action and every argument, whole, as (label, value) rows; page.rs gives
 * it the Publisher decoded from the calldata signed. Pure: NOX amounts
 * come through `nox`, addresses through `k` and `at`.
 */

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use nonos_marketplace_abi::Keccak;

use super::call::Publisher;
use super::review_bond::{bond_rows, At, Rows};
use super::text::hex;

pub fn review_rows(p: &Publisher, k: &impl Keccak, at: At, nox: fn(u128) -> String) -> Rows {
    use Publisher::*;
    let s = |t: &str| t.to_string();
    match p {
        Publish { publisher_key, app_id_hash, release_id, release_hash, bond_id, uri } => {
            Vec::from([
                ("action", s("publish a release")),
                ("publisher key", hex(publisher_key)),
                ("app", hex(app_id_hash)),
                ("release", hex(release_id)),
                ("release hash", hex(release_hash)),
                ("bond", hex(bond_id)),
                ("fetched from", String::from_utf8_lossy(uri).into_owned()),
            ])
        }
        WithdrawRelease { listing_id, release_id } => Vec::from([
            ("action", s("withdraw a release")),
            ("listing", hex(listing_id)),
            ("release", hex(release_id)),
        ]),
        Anchor { anchor, kind, listing_id } => Vec::from([
            ("action", s("anchor a proof on chain")),
            ("anchor", hex(anchor)),
            ("proof kind", format!("{kind}")),
            ("listing", hex(listing_id)),
            ("commits to", s("a hash, not a location: nothing is fetched from it")),
        ]),
        _ => bond_rows(p, k, at, nox).unwrap_or_default(),
    }
}
