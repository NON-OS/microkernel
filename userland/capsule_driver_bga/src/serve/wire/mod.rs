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

mod header;
mod ops;
mod parse;
mod read;
mod reply;

pub use header::RX_LEN;
pub use ops::{
    E_BAD_OP, E_BUSY, E_DEVICE, E_INVAL, FLUSH_LEN, OP_FLUSH, OP_GET_PRIMARY_SURFACE,
    OP_SET_SCANOUT, OP_TRANSFER_TO_HOST, PRIMARY_LEN, SET_SCANOUT_LEN, TRANSFER_LEN,
};
pub use parse::{parse, refused};
pub use read::{le_u32, le_u64};
pub use reply::{payload, status, Reply, REPLY_LEN};
