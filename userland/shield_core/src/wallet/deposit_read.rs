//! Reading the pool state a deposit depends on, over the wallet's own Tor.
//!
//! Ten view calls, twelve on a launch pool, each decoded strictly: a reply that is not one ABI
//! word is refused, never read as zero, since a zero cap and a garbled reply mean different things.

#[path = "deposit_read_abi.rs"]
mod abi;
#[path = "deposit_read_checked.rs"]
mod checked;
#[cfg(feature = "fuzzing")]
#[path = "deposit_read_fuzz.rs"]
pub mod fuzz;
#[path = "deposit_read_selectors.rs"]
mod selectors;
#[path = "deposit_read_shape.rs"]
mod shape;
#[path = "deposit_read_state.rs"]
mod state;

pub use checked::read_pool_state_checked;
pub use state::read_pool_state;
