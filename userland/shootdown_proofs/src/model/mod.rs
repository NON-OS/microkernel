// NONOS Operating System (AGPL-3.0-or-later)
//! One shootdown round, the CPUs it must reach, and the APs that come up
//! while it runs, as a state machine small enough to search exhaustively.
//!
//! The model follows `src/memory/paging/manager/shootdown/` and the AP path
//! in `src/smp/init/ap_unit.rs` and `src/smp/ap/`. One kernel page changes
//! from `Old` to `New`. CPU 0 runs the round. Every other CPU is an AP that is
//! either online when the round starts or is brought up, or given up on,
//! while it runs. Each step is one atomic action of the code it names, and the
//! search takes every order the steps can run in.
//!
//! Memory is sequentially consistent here. The code makes it so with the
//! `SeqCst` fence before `select` and the one between setting a target's
//! pending flag and sending its IPI (`broadcast.rs`, `send.rs`), and with the
//! `SeqCst` stores of a descriptor's state.
//!
//! The hardware may cache: a CPU running kernel code may load the page's
//! translation into its TLB at any step and drop it at any step. INIT empties
//! a TLB (Intel SDM, vol. 3A, 9.1), so an AP woken by INIT and STARTUP starts
//! with nothing cached, and so does one the boot CPU gives up on and parks.

mod ap;
mod check;
mod config;
mod hardware;
mod initiator;
mod state;
mod state_ops;
mod step;
mod target;
mod tlb;
mod types;
mod waiter;

pub use check::check;
pub use config::{Config, MAX_CPUS};
pub use state::{Ap, Handler, Init, State};
pub use step::successors;
pub use types::{Flush, Pge, Step, Val, BRING_UP};
