// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 boot path's memory: the DMA pool placement,
//! the allocator's span and the free RAM a NONOS loader's EFI map names.

// Test builds only: the kernel keeps `place` private to its boot path, so a
// library build would have nothing that calls it.
#[cfg(test)]
#[path = "../../../src/arch/aarch64/boot/memory/clip.rs"]
mod clip;

#[cfg(test)]
#[path = "../../../src/arch/aarch64/boot/memory/dma_place.rs"]
mod dma_place;

#[cfg(test)]
#[path = "../../../src/arch/aarch64/boot/memory/efi_runs.rs"]
mod efi_runs;

#[cfg(test)]
#[path = "../../../src/arch/aarch64/boot/memory/span.rs"]
mod span;

#[cfg(test)]
#[path = "../../../src/arch/aarch64/boot/memory/usable.rs"]
mod usable;

#[cfg(test)]
mod brute_tests;
#[cfg(test)]
mod clip_tests;
#[cfg(test)]
mod efi_runs_tests;
#[cfg(test)]
mod high_half_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod span_tests;
#[cfg(test)]
mod usable_brute_tests;
#[cfg(test)]
mod usable_tests;
