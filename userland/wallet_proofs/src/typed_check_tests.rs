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

//! What is refused before the key is touched, and what after.

use nonos_nox_config::{MAINNET, SEPOLIA};

use crate::server::typed::check::{admit, caller_allowed, signer_rule};
use crate::server::typed::entry::Entry;
use crate::server::typed::refusal::Refusal;
use crate::server::typed::table::ENTRIES;

#[test]
fn nothing_signs_while_no_contract_is_deployed() {
    for e in ENTRIES {
        let words = vec![0u8; e.fields.len() * 32];
        for chain in [MAINNET, SEPOLIA] {
            assert_eq!(admit(e, chain, &words), Err(Refusal::NotDeployed), "{}", e.id);
        }
    }
}

#[test]
fn only_mainnet_and_sepolia() {
    for e in ENTRIES {
        for chain in [0, 5, 10, 137, 8453, u64::MAX] {
            assert_eq!(admit(e, chain, &[]), Err(Refusal::Chain), "{} on {chain}", e.id);
        }
    }
}

#[test]
fn callers_are_named_by_the_registry() {
    let voucher = ENTRIES[0];
    let owner = |name: &[u8]| match name {
        b"payment" => Some(40),
        b"app.nonos_wallet.1" => Some(41),
        _ => None,
    };
    assert!(caller_allowed(voucher, 40, owner));
    assert!(!caller_allowed(voucher, 41, owner));
    assert!(caller_allowed(ENTRIES[1], 41, owner) && caller_allowed(ENTRIES[2], 41, owner));
    assert!(!caller_allowed(ENTRIES[1], 40, owner));
    assert!(!caller_allowed(voucher, 0, |_| Some(0)));
    assert!(!caller_allowed(voucher, 40, |_| None));
}
#[test]
fn the_voucher_payer_is_the_signer() {
    let v: &Entry = ENTRIES[0];
    let signer = [0xAB; 20];
    let mut words = vec![0u8; v.fields.len() * 32];
    assert_eq!(signer_rule(v, &words, &signer), Err(Refusal::NotSigner));
    words[32 + 12..64].copy_from_slice(&signer);
    assert_eq!(signer_rule(v, &words, &signer), Ok(()));
    words[2 * 32 + 12..3 * 32].copy_from_slice(&[0xCD; 20]);
    assert_eq!(signer_rule(v, &words, &signer), Ok(()));
    assert_eq!(signer_rule(ENTRIES[1], &[0; 96], &signer), Ok(()));
}
