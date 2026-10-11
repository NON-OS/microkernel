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

//! INSTALL_READY's last byte: why the bond gate stands
//! as it does. 0 is Unread, so a zero from anywhere still means "not read";
//! only Unread is "could not be read", and each failure has its own
//! sentence.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum BondCode {
    /// No reading under an hour old on the chain asked, or no chain asked.
    Unread = 0,
    /// Free, or no commerce block: no bond is asked for.
    NotNeeded = 1,
    Active = 2,
    /// Another payee's or another publisher key's bond.
    NotThisListing = 3,
    /// Not active, or unbonding.
    Inactive = 4,
    BelowMinimum = 5,
    /// Escrow at a price above what the bond can cover.
    PriceAboveBond = 6,
}

impl BondCode {
    /// None for a byte no market writes.
    pub fn from_byte(b: u8) -> Option<BondCode> {
        use BondCode::*;
        [Unread, NotNeeded, Active, NotThisListing, Inactive, BelowMinimum, PriceAboveBond]
            .get(b as usize)
            .copied()
    }

    pub fn passes(self) -> bool {
        matches!(self, BondCode::NotNeeded | BondCode::Active)
    }

    /// What a person reads when the bond gate fails; None when it passes.
    pub fn sentence(self) -> Option<&'static [u8]> {
        Some(match self {
            BondCode::NotNeeded | BondCode::Active => return None,
            BondCode::Unread => b"Seller's bond could not be read",
            BondCode::NotThisListing => b"Seller's bond is another payee's or publisher key's",
            BondCode::Inactive => b"Seller's bond is not active, or is unbonding",
            BondCode::BelowMinimum => b"Seller's bond is below the market's minimum",
            BondCode::PriceAboveBond => b"Its escrow price is more than the seller's bond covers",
        })
    }
}
