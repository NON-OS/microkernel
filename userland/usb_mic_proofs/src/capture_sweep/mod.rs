// NONOS Operating System (AGPL-3.0-or-later)
//! A seeded sweep over captured sound: isochronous batches, read into the
//! ring a packet at a time and taken out in whole frames.

pub mod build;
pub mod lie;
pub mod lie_batch;
pub mod lies;
pub mod model;
pub mod rng;
