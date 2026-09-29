// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/serial/ops/io.rs"]
pub mod io;

#[path = "../../../../../../../../../src/arch/x86_64/serial/ops/read_write.rs"]
pub mod read_write;

pub use io::{is_data_ready, is_tx_empty, read_byte_direct, read_reg, write_byte_timeout, write_reg};
pub use read_write::{available, available_from_port, is_port_initialized, module_is_initialized, read_byte, read_byte_direct_from_port, read_byte_from_port, write_byte, write_byte_to_port, write_str, write_str_to_port};
