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

mod codec;
mod header;
mod ops;

pub use codec::{le_u32, parse, refused, response_header, write_status};
pub use header::{Request, E_BAD_OP, E_BUSY, E_DEVICE, E_INVAL, HDR_LEN, STATUS_LEN};
pub use ops::{
    OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_HEALTHCHECK, OP_SET_SCANOUT, OP_TRANSFER_TO_HOST,
};
