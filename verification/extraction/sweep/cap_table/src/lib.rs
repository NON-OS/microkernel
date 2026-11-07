// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root for `src/syscall/contract/cap_table`.
//!
//! The two top-level modules point at the kernel's `src/capabilities` and
//! `src/syscall`, and the inline modules below them name only the files the
//! table reads, so each resolves `crate::...` exactly as it does in the
//! kernel. The kernel clock is the one
//! item not included: it is declared without a body, so it is an opaque
//! function in the extracted Lean and a named row of the assumptions
//! register. A process token carries no expiry, so its checks never call it.

pub mod time {
    unsafe extern "Rust" {
        /// `crate::time::timestamp_millis`, the kernel's monotonic clock.
        pub safe fn timestamp_millis() -> u64;
    }
}

#[path = "../../../../../src/capabilities"]
pub mod capabilities {
    pub mod types {
        mod table;
        mod defs;
        pub use defs::Capability;
    }
    pub mod token {
        pub mod types {
            mod defs;
            mod authority_admin;
            mod authority_broker;
            mod query;
            pub use defs::CapabilityToken;
        }
    }
    pub use token::types::CapabilityToken;
    pub use types::Capability;

    /// `Capability::bit`, the capability's place in a `caps_bits` word. A
    /// Charon entry point cannot name an inherent method; this adds no logic.
    pub fn capability_bit(c: Capability) -> u64 {
        c.bit()
    }
}

#[path = "../../../../../src/syscall"]
pub mod syscall {
    pub mod abi;
    pub mod numbers;
    pub mod caps {
        mod checks;
        pub use crate::capabilities::{Capability, CapabilityToken};
    }
    pub mod contract {
        mod cap_table;

        use crate::capabilities::CapabilityToken;
        use crate::syscall::numbers::SyscallNumber;

        /// `cap_table::is_allowed`. A Charon entry point cannot name a
        /// `pub(super)` function from outside; this adds no logic.
        pub fn is_allowed(caps: &CapabilityToken, number: SyscallNumber, launcher: bool) -> bool {
            cap_table::is_allowed(caps, number, launcher)
        }
    }
}
