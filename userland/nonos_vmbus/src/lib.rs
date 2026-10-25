// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The VMBus channel layer the Hyper-V synthetic drivers share. A driver
//! holds its channel's record (given by the VMBus core, `MkHvChannel`), maps
//! the channel's pages as one DMA grant on it, and through this crate:
//! - lays its two rings out on those pages (`ring`);
//! - hands the host the pages and opens the channel (`channel::open`);
//! - sends and takes packets (`packet`, `Channel::send`, `Channel::recv`);
//! - closes the channel and takes the pages back (`Channel::close`).

#![no_std]

extern crate alloc;
#[cfg(any(test, feature = "model"))]
#[macro_use]
extern crate std;

pub mod channel;
pub mod mem;
pub mod msg;
pub mod packet;
pub mod ring;

/// The host's side, from QEMU's vmbus.c, for this crate's tests and the
/// drivers' proof crates.
#[cfg(any(test, feature = "model"))]
pub mod model;

#[cfg(test)]
mod tests;

pub use channel::{gpadl, open, teardown, Channel, Host, Layout, OpenError, PostError};
pub use mem::{Buffer, Pages, Shared};
pub use packet::Header;
pub use ring::{Ring, RingError};
