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

//! What stands in for the kernel around the mounted files. Each keeps what
//! it saw per thread, so a test reads only what its own calls did.

pub mod block_io;
pub mod devices;
pub mod kernel;
pub mod keygen;
pub mod keyring;
pub mod pqc;
pub mod ram;
pub mod rng;
pub mod selftest;
pub mod time;
pub mod volume;

pub use devices::hardware;
pub use kernel::{arch, boot, log, smp, sys};
pub use keyring::{audit, keyring_capsule, machine_key, scrub_asm};
pub use ram::memory;
pub use volume::blockfs;
