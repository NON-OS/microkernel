// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/capabilities/token/types/construct.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod capabilities;

pub fn capabilitytoken_empty() -> crate::capabilities::token::types::defs::CapabilityToken {
    crate::capabilities::token::types::defs::CapabilityToken::empty()
}

pub fn capabilitytoken_system() -> crate::capabilities::token::types::defs::CapabilityToken {
    crate::capabilities::token::types::defs::CapabilityToken::system()
}

