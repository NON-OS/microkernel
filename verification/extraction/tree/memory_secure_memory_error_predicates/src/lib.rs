// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/secure_memory/error/predicates.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn securememoryerror_is_security_critical(this: crate::memory::secure_memory::error::types::SecureMemoryError) -> bool {
    this.is_security_critical()
}

pub fn securememoryerror_is_internal_error(this: crate::memory::secure_memory::error::types::SecureMemoryError) -> bool {
    this.is_internal_error()
}

pub fn securememoryerror_is_retriable(this: crate::memory::secure_memory::error::types::SecureMemoryError) -> bool {
    this.is_retriable()
}

