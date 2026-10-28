// NONOS Operating System (AGPL-3.0-or-later)

//! The real libc's broker records, so the stubs take the layouts it ships.

#[path = "../../../libc/src/broker/types/bar.rs"]
pub mod bar;
#[path = "../../../libc/src/broker/types/bus.rs"]
pub mod bus;
#[path = "../../../libc/src/broker/types/device.rs"]
pub mod device;
#[path = "../../../libc/src/broker/types/dma.rs"]
pub mod dma;
#[path = "../../../libc/src/broker/types/mmio.rs"]
pub mod mmio;
