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

//! Why a listing will not install, named (B9, the plan): the first gate of
//! INSTALL_READY's eleven that failed, as a sentence a person can act on.
//! The bond's failure is named by its code (v1.4.35); an Unread bond is
//! "checking" while the store still asks again (#60), and only then "could
//! not be read".

use nonos_market_proto::BondCode;

use super::market::Readiness;
use super::verdict::BOND_GATE;

pub const CHECKING: &[u8] = b"Checking the seller's bond on chain";
pub const BOND_UNREAD: &[u8] = b"Seller's bond could not be read";
/// A bond gate that failed with a code that names no failure.
const BOND_FAILED: &[u8] = b"Seller's bond did not pass the market's check";

const WHY: [&[u8]; 11] = [
    b"The market index's signature did not verify",
    b"The market names no package to fetch",
    b"The publisher's signature did not verify",
    b"The operator has not validated this release",
    b"It is built for another kind of machine",
    b"It carries no proof the system accepts",
    b"Its price and payee are not confirmed by both publisher and payee",
    b"Its payee has not attested this release",
    b"It has no guest policy the market accepts",
    b"Its shown price contradicts what it sells",
    BOND_UNREAD,
];

/// The first failed gate's sentence; None when it is ready. `checking`: the
/// store is still asking again while the market reads the bond.
pub fn reason(r: &Readiness, checking: bool) -> Option<&'static [u8]> {
    if r.install_ready {
        return None;
    }
    let first = r.gates.iter().position(|ok| !ok)?;
    Some(match first {
        BOND_GATE if checking && r.bond == BondCode::Unread => CHECKING,
        BOND_GATE => r.bond.sentence().unwrap_or(BOND_FAILED),
        i => WHY.get(i).copied()?,
    })
}

/// The mark on a gate's row: pass, fail, or for an unread bond "not
/// checked".
pub fn mark(gate: usize, pass: bool, bond: BondCode) -> &'static [u8] {
    match (pass, gate) {
        (true, _) => b"pass",
        (false, BOND_GATE) if bond == BondCode::Unread => b"not checked",
        (false, _) => b"fail",
    }
}
