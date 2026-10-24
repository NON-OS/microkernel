// NONOS Operating System (AGPL-3.0-or-later)
//! The model, stated apart from the code, from the kernel's own gates:
//! - a capsule reaches the network only through `Network`; storage only
//!   through `FileSystem`, `StoreWrite` or `StreamImport`; devices only
//!   through `Driver`, `Mmio`, `Irq`, `Dma` or `Pio`. `Admin`, `Hardware`
//!   and `DeviceEnum` reach none (`src/syscall/caps/checks`,
//!   `src/capabilities/token/types/authority_broker.rs`);
//! - every process the kernel lists, and every registry entry, is one row,
//!   ordered by pid; pid 0 is the kernel's; a row with no registry entry
//!   has no signed ceiling and claims no reach;
//! - Admin is held only by init, policy, acpi, power, app.install and
//!   tool.install; no holder of Debug also holds InputSource,
//!   InputConsumer, Keyring or DeviceSecret.

mod format_tests;
mod fuzz_tests;
mod holders_tests;
mod join_tests;
mod model;
mod names_tests;
mod names_unknown_tests;
mod reach_tests;
mod view_tests;
mod withheld_tests;
