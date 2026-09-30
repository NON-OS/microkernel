// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/security/capsule_manifest/schema/endpoint.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod security;

pub fn endpointkind_as_u8(this: crate::security::capsule_manifest::schema::endpoint::EndpointKind) -> u8 {
    this.as_u8()
}

