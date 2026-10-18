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

//! Row 1.3: a candidate is taken only if the session's own policy selects
//! from it exactly the `Selection` the session runs, and it names the same
//! group management cipher. One field changed at a time, each refused.

use super::select::*;
use nonos_wifi_core::rsn::{Ciphers, JoinPolicy};
use nonos_wifi_core::softmac::roam::roam_policy;

const PSK: u8 = 2;
const SAE: u8 = 8;
const CAPABLE: u8 = 0x80;
const REQUIRED: u8 = 0xc0;

#[test]
fn the_same_security_is_taken() {
    let r = rsne(CCMP, CCMP, PSK, CAPABLE, Some(BIP_CMAC));
    assert!(picked(&r, &r));
}

#[test]
fn another_pairwise_or_group_cipher_is_refused() {
    let ours = rsne(CCMP, CCMP, PSK, 0, None);
    assert!(!picked(&ours, &rsne(CCMP, GCMP256, PSK, 0, None)), "GCMP-256 against CCMP");
    assert!(!picked(&ours, &rsne(GCMP256, CCMP, PSK, 0, None)), "another group cipher");
}

#[test]
fn protection_required_against_capable_is_refused() {
    let ours = rsne(CCMP, CCMP, PSK, CAPABLE, None);
    assert!(!picked(&ours, &rsne(CCMP, CCMP, PSK, REQUIRED, None)));
    assert!(!picked(&rsne(CCMP, CCMP, PSK, REQUIRED, None), &ours), "and the other way");
}

#[test]
fn another_group_management_cipher_is_refused() {
    let ours = rsne(CCMP, CCMP, PSK, CAPABLE, Some(BIP_CMAC));
    assert!(!picked(&ours, &rsne(CCMP, CCMP, PSK, CAPABLE, Some(BIP_GMAC256))));
}

#[test]
fn hash_to_element_against_hunting_and_pecking_is_refused() {
    let sae = rsne(CCMP, CCMP, SAE, REQUIRED, None);
    let h2e = [sae.clone(), RSNXE_H2E.to_vec()].concat();
    assert!(picked(&h2e, &h2e));
    assert!(!picked(&h2e, &sae), "H2E against hunting and pecking");
    assert!(!picked(&sae, &h2e), "and the other way");
}

#[test]
fn a_roam_policy_is_the_sessions_never_the_persons_wider_one() {
    let (psk, _) = session(&rsne(CCMP, CCMP, PSK, 0, None));
    let (sae, _) = session(&rsne(CCMP, CCMP, SAE, REQUIRED, None));
    let c = Ciphers::CCMP_128_ONLY;
    let (p, s) = (roam_policy(&psk, c), roam_policy(&sae, c));
    assert!((p.allow_psk, p.allow_sae) == (true, false) && p != JoinPolicy::ANY);
    assert!((s.allow_psk, s.allow_sae) == (false, true) && s != JoinPolicy::ANY);
    let mixed = [rsne(CCMP, CCMP, PSK, 0, None)].concat();
    assert!(!picked(&rsne(CCMP, CCMP, SAE, REQUIRED, None), &mixed), "SAE never roams to PSK");
}
