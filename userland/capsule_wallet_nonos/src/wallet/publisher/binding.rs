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
 * Entry 2, PublisherBinding, from an entry document (plan/31): what the
 * screen shows (the publisher key in full, the listing string, the payee)
 * and the words it signs once the person confirms the key. The entry must
 * sell through this account (commerce.payee) and name this account's own
 * bond for that key (commerce.bond_id == bondId(signer, publisherKey)).
 */

use alloc::string::String;
use alloc::vec::Vec;

use nonos_marketplace_abi::{listing_id, Keccak, MarketplaceEntry};

use super::super::hex_id::hex;
use super::super::typed::{binding_words, Wrong};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refused {
    /// No commerce block: nothing is sold, so nothing is bound.
    NotSold,
    /// The entry pays another account.
    NotPayee,
    /// The entry's bond is not this account's for its publisher key.
    NotOurBond,
    /// A release form plan/23 does not allow.
    Shape,
    /// No release with that index.
    NoRelease,
}

pub struct BindingPlan {
    pub rows: Vec<(&'static str, String)>,
    pub words: [[u8; 32]; 3],
}

pub fn binding_plan(
    k: &impl Keccak,
    e: &MarketplaceEntry,
    signer: &[u8; 20],
) -> Result<BindingPlan, Refused> {
    let c = e.commerce.as_ref().ok_or(Refused::NotSold)?;
    if c.payee != *signer {
        return Err(Refused::NotPayee);
    }
    let listing = listing_id(k, &e.publisher_pubkey, &e.listing_id);
    let words = binding_words(k, signer, &e.publisher_pubkey, &listing, &c.bond_id).map_err(
        |w| match w {
            Wrong::NotOurBond => Refused::NotOurBond,
            Wrong::Shape => Refused::Shape,
        },
    )?;
    let rows = Vec::from([
        ("publisher key", hex(&e.publisher_pubkey)),
        ("listing", e.listing_id.clone()),
        ("payee", hex(signer)),
    ]);
    Ok(BindingPlan { rows, words })
}
