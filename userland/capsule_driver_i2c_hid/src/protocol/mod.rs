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

mod decode;
mod encode;
mod errno;
mod header;
mod limits;
mod ops;

pub use decode::{parse, received, refused};
pub use encode::response;
pub use errno::{E_BAD_OP, E_INVAL, E_NOT_FOUND, E_OK};
pub use header::{Request, HDR_LEN};
pub use limits::IPC_PAYLOAD_MAX;
pub use ops::{OP_DESCRIPTOR, OP_HEALTHCHECK};
