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
 * Why a publisher call is not built from the entry. Every one is first
 * held to binding_plan's checks: the entry sells, pays this account, and
 * names this account's own bond for its publisher key.
 */

use nonos_marketplace_abi::{Commerce, Keccak, MarketplaceEntry};

use super::super::publisher::{binding_plan, Refused};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Why {
    /// The entry itself: not sold, another payee, not our bond, no release.
    Entry(Refused),
    /// Below the Bond's MIN_BOND (READ 33), or 0.
    BelowMinBond,
    /// An escrow listing's bond below its commerce unit_price.
    BelowUnitPrice,
    /// The typed text is not one address.
    NotAnAddress,
    /// Mixed case that is not the address's EIP-55 spelling.
    Checksum,
    /// The zero address, which delivers nothing.
    ZeroAgent,
    /// bondOf's unlockAt is 0 or still ahead (READ 30).
    Locked,
    /// The Bond holds no credit for this account (READ 35).
    NoCredit,
    /// Not https, past MAX_URL, or with a user, password or port.
    Uri,
    /// A schema 2 release, never signed under v3.
    Schema2,
}

impl From<Refused> for Why {
    fn from(r: Refused) -> Self {
        Why::Entry(r)
    }
}

/// The entry's commerce once binding_plan takes it for `me`.
pub(super) fn ours<'e>(
    k: &impl Keccak,
    e: &'e MarketplaceEntry,
    me: &[u8; 20],
) -> Result<&'e Commerce, Why> {
    binding_plan(k, e, me)?;
    e.commerce.as_ref().ok_or(Why::Entry(Refused::NotSold))
}
