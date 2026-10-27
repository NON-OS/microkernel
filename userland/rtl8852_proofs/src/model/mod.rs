// NONOS Operating System (AGPL-3.0-or-later)
//! The modelled RTL8852BE: a register file that answers the way rtw89 says
//! the chip does where the driver waits on it (`on_read`, `on_write`), the
//! XTAL SI bus and efuse behind it, the RF registers behind SWSI, the
//! firmware CPU (download, H2C commands, the register mailbox), the PCI DMA
//! engine over host memory (`bus`), and the air to a simulated AP. Anything
//! the driver does that the chip would not take is written to
//! `violations`, which every proof expects empty unless it is the point.

pub mod air;
pub mod bus;
pub mod card;
pub mod chans;
pub mod cpu;
pub mod dbi;
pub mod efuse_img;
pub mod faults;
mod frames;
mod h2c_seen;
mod mailbox;
mod on_read;
mod on_write;
mod rf;
pub mod rig;
pub mod rxbuf;
pub mod rxq;
pub mod state;
mod state_new;
pub mod txq;
mod xtal;

pub use card::Card;
pub use faults::Faults;
pub use rf::tuned as rf_tuned;
pub use state::State;
