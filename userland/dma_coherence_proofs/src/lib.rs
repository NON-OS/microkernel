// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for non-coherent DMA.

#[cfg(test)]
#[path = "../../../src/arch/cache_span.rs"]
pub mod cache_span;
#[path = "../../../src/arch/aarch64/dma/carve.rs"]
pub mod carve;
#[path = "../../libc/src/broker/dma_lines.rs"]
pub mod dma_lines;

#[cfg(test)]
mod carve_tests;
#[cfg(test)]
mod model;
#[cfg(test)]
mod model_tests;
#[cfg(test)]
mod order_tests;
#[cfg(test)]
mod span_tests;
