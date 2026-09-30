// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/syscall/microkernel/pio/unsupported.rs"]
pub mod unsupported;

pub use unsupported::{sys_pio_grant, sys_pio_read, sys_pio_release, sys_pio_write};
