// NONOS Operating System (AGPL-3.0-or-later)

//! The access points a join may pick from, chosen by security and signal.

mod candidate;
mod list;
mod next;
mod take;

pub use candidate::Candidate;
pub use list::{Candidates, FRAME_MAX, MAX_CANDIDATES, MAX_TRIES};
pub use next::moves_on;
