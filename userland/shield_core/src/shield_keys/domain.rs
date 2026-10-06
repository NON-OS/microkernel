//! The derivation contexts, each named once with its purpose and version.
//! Changing a string changes every key under it, and the wallet can no longer find its notes.

/// Derivation contexts, so no two keys from one seed are reachable from each other.
pub const SPEND_CONTEXT: &str = "nox-shield 2026 spend key v1";

/// The viewing secret. It decrypts incoming notes and cannot spend.
pub const VIEW_CONTEXT: &str = "nox-shield 2026 view key v1";

/// The note store's row key. A copied store is sealed rows whose key lives only in the seed.
pub const STORE_CONTEXT: &str = "nox-shield 2026 note store key v1";

/// Address checksum tag, so an address from another network or version fails to decode.
pub const ADDRESS_TAG: &[u8] = b"nox-shield/address/v1";

/// The X-Wing receiving seed, kept apart from the spend secret so receiving never grants spending.
pub const RECEIVE_CONTEXT: &str = "nox-shield 2026 receive key v1";
/// The receiving address text checksum. A new tag, so the old two-key address never decodes here.
pub const RECEIVE_ADDRESS_TAG: &[u8] = b"nox-shield/address/v2";
/// The view key text checksum, with the prefix hashed in so an incoming key never reads as full.
pub const VIEW_KEY_TAG: &[u8] = b"nox-shield/viewkey/v1";
