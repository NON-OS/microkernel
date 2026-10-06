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

//! The attest service's own frame codec, included by path, so the board's
//! frames are checked against the decoder that will read them.

#[path = "../../../capsule_attest/src/protocol/decode.rs"]
mod decode;
#[path = "../../../capsule_attest/src/protocol/encode.rs"]
mod encode;
#[path = "../../../capsule_attest/src/protocol/errno.rs"]
#[allow(dead_code)]
mod errno;
#[path = "../../../capsule_attest/src/protocol/header.rs"]
mod header;
#[path = "../../../capsule_attest/src/protocol/ops.rs"]
#[allow(dead_code)]
mod ops;

pub use decode::parse;
pub use encode::{response_header, write_status};
pub use errno::{E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION};
pub use header::{Request, HDR_LEN, MAGIC, VERSION};
pub use ops::{OP_PROOF_ROUTE, OP_ROUTE_REPORT};
