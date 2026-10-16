// NONOS Operating System (AGPL-3.0-or-later)

//! Building a commit and deriving its keys.

use p256::{ProjectivePoint, Scalar};

use super::super::group::{point_xy, scalar_bytes, scalar_from_bytes, scalar_is_trivial, LEN};
use super::types::{Commit, SaeKeys};
use crate::wpa::kdf::kdf_sha256;
use crate::wpa::sha256::hmac_sha256;

impl Commit {
    /// Build a commit from the PWE and two 32-byte random values for rand and
    /// mask. `None` if either is not a scalar in (1, r) or their sum is not
    /// (the caller draws again).
    pub fn new(pwe: ProjectivePoint, rand: &[u8; LEN], mask: &[u8; LEN]) -> Option<Commit> {
        let rand = scalar_from_bytes(rand)?;
        let mask = scalar_from_bytes(mask)?;
        if scalar_is_trivial(&rand) || scalar_is_trivial(&mask) {
            return None;
        }
        let scalar = rand + mask;
        if scalar_is_trivial(&scalar) {
            return None;
        }
        let element = -(pwe * mask);
        Some(Commit { rand, scalar, element, pwe })
    }

    /// The commit-scalar as sent.
    pub fn scalar(&self) -> [u8; LEN] {
        scalar_bytes(&self.scalar)
    }

    /// The COMMIT-ELEMENT as sent, x || y. `None` only for the identity, which
    /// a mask in (1, r) times a valid PWE never is.
    pub fn element(&self) -> Option<[u8; 2 * LEN]> {
        point_xy(&self.element)
    }

    /// The keys from the peer's commit. `None` if K is the identity element.
    pub fn derive_keys(&self, peer_scalar: &Scalar, peer_element: &ProjectivePoint) -> Option<SaeKeys> {
        let k_point = (self.pwe * peer_scalar + peer_element) * self.rand;
        let mut kxy = point_xy(&k_point)?;
        let mut keyseed = hmac_sha256(&[0u8; 32], &kxy[..LEN]);
        let context = scalar_bytes(&(self.scalar + peer_scalar));
        let mut both = [0u8; 64];
        let _ = kdf_sha256(&keyseed, b"SAE KCK and PMK", &context, &mut both);
        let mut keys = SaeKeys { kck: [0u8; 32], pmk: [0u8; 32], pmkid: [0u8; 16] };
        keys.kck.copy_from_slice(&both[..32]);
        keys.pmk.copy_from_slice(&both[32..]);
        keys.pmkid.copy_from_slice(&context[..16]);
        super::super::wipe(&mut kxy);
        super::super::wipe(&mut keyseed);
        super::super::wipe(&mut both);
        Some(keys)
    }
}
