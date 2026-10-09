//! The note key and view tag, derived from a note's shared secret.
//! Binding both public halves into the key stops replay under another ephemeral, which keeps the
//! fixed nonce safe. The view tag is never sent: a server holding it would know which are ours.

use blake3::Hasher;

/// The context for a note's symmetric key. Named and versioned so a future
/// format cannot be opened by this one's keys.
const KEY_CONTEXT: &str = "nox-shield 2026 note key v1";

/// The context for the view tag, the byte a scan checks before an AEAD open.
const TAG_CONTEXT: &str = "nox-shield 2026 note tag v1";

/// Derive the note key from the shared secret and both public halves.
pub fn note_key(shared: &[u8; 32], eph_pk: &[u8; 32], view_pk: &[u8; 32]) -> [u8; 32] {
    let mut h = Hasher::new_derive_key(KEY_CONTEXT);
    h.update(shared);
    h.update(eph_pk);
    h.update(view_pk);
    *h.finalize().as_bytes()
}

/// The view tag. A scan opens the AEAD only on a match, about one output in 256.
pub fn view_tag(shared: &[u8; 32], eph_pk: &[u8; 32]) -> u8 {
    let mut h = Hasher::new_derive_key(TAG_CONTEXT);
    h.update(shared);
    h.update(eph_pk);
    h.finalize().as_bytes()[0]
}
