// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/paging/error/impls_recover.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn pagingerror_is_recoverable(this: crate::memory::paging::error::types::PagingError) -> bool {
    this.is_recoverable()
}

pub fn pagingerror_is_demand_pageable(this: crate::memory::paging::error::types::PagingError) -> bool {
    this.is_demand_pageable()
}

