// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../src/arch/x86_64/serial/constants.rs"]
pub mod constants;

#[path = "../../../../../../../../src/arch/x86_64/serial/types.rs"]
pub mod types;

pub use constants::{COM1_BASE, COM1_IRQ, COM2_BASE, COM2_IRQ, COM3_BASE, COM3_IRQ, COM4_BASE, COM4_IRQ, MAX_COM_PORTS, RX_BUFFER_SIZE, TX_TIMEOUT, UART_CLOCK};
pub use types::{BaudRate, DataBits, Parity, SerialConfig, StopBits};
