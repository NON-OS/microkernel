// NONOS Operating System (AGPL-3.0-or-later)
// The parser keeps `walker/walker.rs` and `property/property.rs`, and its
// `Walker::next` returns a `Result` an `Iterator` could not.
#[allow(clippy::module_inception, clippy::should_implement_trait)]
#[path = "../../../../src/arch/fdt/mod.rs"]
pub mod fdt;

// The boot step that keeps `/chosen` `rng-seed` and erases it from the blob,
// with a recording stand-in for the kernel's seed store, which needs the
// kernel's hashing.
#[cfg(test)]
pub mod aarch64 {
    pub mod boot_entropy {
        use std::sync::Mutex;

        pub static KEPT: Mutex<Vec<Vec<u8>>> = Mutex::new(Vec::new());

        pub fn keep(seed: &[u8]) {
            KEPT.lock().unwrap().push(seed.to_vec());
        }
    }
}

#[cfg(test)]
#[path = "../../../../src/arch/aarch64/boot/dtb_adapter/parse/entropy.rs"]
pub mod seed_step;
