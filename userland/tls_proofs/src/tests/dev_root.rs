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

//! The local test chain's throwaway root (P5) as nonos_tls's devnet module
//! holds it, against the files tools/devnet/tls commits. A build without the
//! feature mounts the module here, so these hold whether or not the crate's
//! store takes it; dev_root_trust proves which builds do.

use p256::ecdsa::SigningKey;
use sha2::{Digest, Sha256};

use crate::cert_dns_match::matches;
use crate::cert_is_ca::cert_is_ca;
use crate::cert_spki::cert_spki;

#[cfg(not(feature = "devnet"))]
#[path = "../../../nonos_tls/src/roots/dev.rs"]
mod dev;
#[cfg(feature = "devnet")]
use crate::roots::dev;

pub(super) static ROOT: &[u8] = include_bytes!("../../../../tools/devnet/tls/root.der");
pub(super) static LEAF: &[u8] = include_bytes!("../../../../tools/devnet/tls/leaf.der");
static LEAF_KEY: &[u8] = include_bytes!("../../../../tools/devnet/tls/leaf-key.throwaway.der");
pub(super) const HOST: &[u8] = b"rpc.devnet.nonos.test";
pub(super) const NOW: u64 = 20261025000000; // both run 2026-01-01 to 2045-12-31

#[test]
fn the_compiled_root_is_the_committed_one() {
    assert_eq!(Sha256::digest(dev::DEV_ROOT_SPKI)[..], dev::DEV_ROOT_SPKI_SHA256[..]);
    assert_eq!(cert_spki(ROOT), Some(dev::DEV_ROOT_SPKI));
    assert_eq!(crate::cert_issuer::cert_issuer(LEAF), Some(dev::DEV_ROOT_SUBJECT));
    assert!(dev::DEV_ROOT_SUBJECT.windows(27).any(|w| w == b"NONOS-devnet-throwaway-root"));
}
#[test]
fn the_root_signs_a_leaf_for_the_local_rpc_alone() {
    assert!(cert_is_ca(ROOT) && !cert_is_ca(LEAF));
    assert_eq!(crate::verify_link::verify_link(LEAF, dev::DEV_ROOT_SPKI), Ok(true));
    assert!([LEAF, ROOT].iter().all(|c| crate::cert_valid_now::cert_valid_now(c, NOW)));
    assert!(matches(LEAF, HOST));
    for host in ["rpc.devnet.nonos.test.evil.example", "evil.rpc.devnet.nonos.test", "a.test"] {
        assert!(!matches(LEAF, host.as_bytes()), "{host}");
    }
}
#[test]
fn the_module_answers_for_its_own_root_only() {
    assert!(dev::is_dev_root(&dev::DEV_ROOT_SPKI_SHA256));
    let mut near = dev::DEV_ROOT_SPKI_SHA256;
    near[31] ^= 1;
    assert!(!dev::is_dev_root(&near));
    assert_eq!(dev::dev_spki_by_subject(dev::DEV_ROOT_SUBJECT), Some(dev::DEV_ROOT_SPKI));
    let cut = &dev::DEV_ROOT_SUBJECT[..dev::DEV_ROOT_SUBJECT.len() - 1];
    assert_eq!(dev::dev_spki_by_subject(cut), None);
}
#[test]
fn the_proxy_key_is_the_leaf_key() {
    /* PKCS#8 for P-256 as generate.py writes it: the scalar's OCTET STRING at 34. */
    assert_eq!(LEAF_KEY[34..36], [0x04, 0x20]);
    let key = SigningKey::from_slice(&LEAF_KEY[36..68]).expect("a P-256 scalar");
    let point = key.verifying_key().to_encoded_point(false);
    assert!(cert_spki(LEAF).expect("spki").ends_with(point.as_bytes()));
}
