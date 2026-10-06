//! Proving, on this device, through the launch prover.
//!
//! The circuit and the prover are the launch package's, vendored whole. This
//! module turns the wallet's own notes and the pool's history into the
//! prover's request, and its proof into what the pool and a relayer read.

mod cancel;
mod hasher;
pub mod launch;

pub use cancel::Cancel;
pub use hasher::pool_hasher;
