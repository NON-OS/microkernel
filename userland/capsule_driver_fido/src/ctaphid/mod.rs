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
pub(crate) mod assemble;
mod assembled;
pub(crate) mod frame;
pub(crate) mod heard;
mod init;
mod job;
mod job_end;
mod key;
mod opening;
mod talk;

pub use frame::{CMD_CANCEL, CMD_CBOR, PACKET};
pub use init::{Channel, CAP_CBOR};
pub use job::{Fault, Job};
pub use key::{find_keys, free_keys, Key};
pub use talk::{allocate, send};
