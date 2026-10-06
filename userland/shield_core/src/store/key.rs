//! The key every stored row is sealed under, derived from the seed so there is a single secret.
//! The associated data pins a row to this store and version, so it cannot be replayed elsewhere.

use crate::custody::Seed;
use crate::keys::material::Material;
use crate::keys::store_key;

/// Associated data on every row, binding it to this store and version.
pub const AAD: &[u8] = b"nox-shield/note-store/v1";

/// The store key of account `index`, derived from the seed already behind the hardware guard.
pub fn derive(seed: &Seed, index: u32) -> [u8; 32] {
    store_key(Material::of(seed, index).bytes())
}
