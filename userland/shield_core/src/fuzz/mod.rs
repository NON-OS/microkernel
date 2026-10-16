// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded, bounded rounds over the readers of what a server or the chain sends: hostile bytes at
//! and past every bound, each round held to what its reader promises. Built only under the
//! `fuzzing` feature; shield_fuzz_proofs runs them in the proof lane, so they cannot rot.

mod hooks;
mod reply;
mod reply_batch;
pub(crate) mod rng;

pub use crate::notes::note_rounds;
pub use crate::wallet::word_rounds;
pub use hooks::hook_rounds;
pub use reply::reply_rounds;
pub use reply_batch::batch_rounds;
