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

use nonos_marketplace_abi::{attest_digest_at, attest_words};
use nonos_marketplace_abi::{CapsuleRelease, MarketplaceEntry, SettleMode};
use nonos_secp256k1::{public_key_from_secret, sign};

use super::binding::BOND_AT;
use super::fixtures::sold;
use super::hosts::{address_of, Host};

/// The fixture's first release, attested by the payee whose secret is 0x22s.
pub(super) fn attested(mode: SettleMode) -> (MarketplaceEntry, CapsuleRelease) {
    let secret = [0x22; 32];
    let mut e = sold().entries[0].clone();
    let c = e.commerce.as_mut().unwrap();
    c.payee = address_of(&public_key_from_secret(&secret).unwrap());
    c.settle_mode = mode;
    let c = c.clone();
    let mut rel = e.releases[0].clone();
    let words = attest_words(&Host, &e, &c, &rel);
    let sig = sign(&secret, &attest_digest_at(&Host, 1, BOND_AT, &words)).unwrap();
    let x = rel.ext.as_mut().unwrap();
    x.release_attest_sig[..32].copy_from_slice(&sig.r);
    x.release_attest_sig[32..64].copy_from_slice(&sig.s);
    x.release_attest_sig[64] = sig.recovery_id + 27;
    (e, rel)
}
