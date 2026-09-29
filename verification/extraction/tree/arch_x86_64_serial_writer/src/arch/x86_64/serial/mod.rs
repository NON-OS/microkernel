// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/x86_64/serial/buffer.rs"]
pub mod buffer;

#[path = "../../../../../../../../src/arch/x86_64/serial/constants.rs"]
pub mod constants;

#[path = "../../../../../../../../src/arch/x86_64/serial/error.rs"]
pub mod error;

pub mod ops;

#[path = "../../../../../../../../src/arch/x86_64/serial/state.rs"]
pub mod state;

#[path = "../../../../../../../../src/arch/x86_64/serial/writer.rs"]
pub mod writer;

pub use constants::{COM1_BASE, COM1_IRQ, COM2_BASE, COM2_IRQ, COM3_BASE, COM3_IRQ, COM4_BASE, COM4_IRQ, MAX_COM_PORTS, RX_BUFFER_SIZE, TX_TIMEOUT, UART_CLOCK};
pub use error::SerialError;
pub use state::{SerialStats, SerialStatsSnapshot};
pub use writer::SerialWriter;
pub use ops::{available, available_from_port, is_port_initialized, module_is_initialized as is_initialized, read_byte, read_byte_direct_from_port, read_byte_from_port, write_byte, write_byte_to_port, write_str, write_str_to_port};
