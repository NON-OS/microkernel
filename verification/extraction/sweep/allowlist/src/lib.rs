// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root for the allowlist's bitmap and `allowed`.
//!
//! `syscall` points at the kernel's `src/syscall`, and the inline modules
//! below it name only the files the allowlist reads. The registry index,
//! `abi::index`, is translated opaque: its hash table is built by `const fn`
//! at compile time, and the theorems hold for any slot lookup and any count.
//! `derive.rs` is left out: `for_token` checks the token's MAC through the
//! crypto code.

#[path = "../../../../../src/syscall"]
pub mod syscall {
    pub mod abi;
    pub mod numbers;
    pub mod contract {
        pub mod allowlist {
            mod decide;
            mod set;
            pub use decide::allowed;
            pub use set::Allowlist;

            /// `Allowlist::set`, `plain`, `launcher` and `EMPTY`. A Charon
            /// entry point cannot name an inherent item; these add no logic.
            pub fn set(list: &mut Allowlist, slot: usize, plain: bool, launcher: bool) {
                list.set(slot, plain, launcher)
            }

            pub fn plain(list: &Allowlist, slot: usize) -> bool {
                list.plain(slot)
            }

            pub fn launcher(list: &Allowlist, slot: usize) -> bool {
                list.launcher(slot)
            }

            pub fn empty() -> Allowlist {
                Allowlist::EMPTY
            }
        }
    }
}
