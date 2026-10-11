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

//! Block reads and writes, and the cache flush.
//!
//! One sector goes as READ_SINGLE_BLOCK or WRITE_BLOCK. More go as
//! SET_BLOCK_COUNT followed by READ_MULTIPLE_BLOCK or WRITE_MULTIPLE_BLOCK,
//! so the card stops on its own count; a card without CMD23 gets Auto CMD12
//! from the host instead. After a write the card's status is polled until
//! it is ready for data in transfer state, so the reply says the data left
//! the bus and the card took it. A failure brings the card back to
//! transfer state before it is reported.

mod flush;
mod plan;
mod run;
mod transfer;
mod wait_ready;

pub use flush::flush;
pub use plan::*;
pub use transfer::transfer;
