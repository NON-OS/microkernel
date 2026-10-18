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

//! The block wire driver.rtsx0 serves, the one nonos_blk_client and the
//! kernel's block clients speak to every disk driver: a 20-byte request
//! header, then the body; the reply echoes the header, then a status word,
//! then the payload. Pure, for the host proofs.

pub mod endpoint;
pub mod errno;
pub mod header;
pub mod identify;
pub mod ops;
pub mod rw;

pub use endpoint::{KERNEL_REPLY_ENDPOINT, SERVICE_NAME};
pub use errno::*;
pub use header::{decode_request, encode_reply, Request, HDR_LEN, STATUS_LEN};
pub use identify::{encode_identify, IDENTIFY_LEN};
pub use ops::*;
pub use rw::parse_rw;
