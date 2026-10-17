/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The address a station joins a network under: one per network, from a
//! per-boot key, so two networks cannot tell they met the same machine, and
//! one network sees one address across its rejoins within a boot. A network
//! is its name and the security it is joined under. One derivation for every
//! Wi-Fi driver; the bits that make an address local and unicast are
//! `nonos_mac`'s.

mod key;
mod kind;
mod one_off;
#[cfg(test)]
mod wipe_tests;

pub use key::{AddressKey, KEY_LEN};
pub use kind::Security;
pub use one_off::one_off;
