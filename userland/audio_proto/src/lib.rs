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

//! The wire format of the audio service, for both ends of it.

#![no_std]

pub mod header;
pub mod ops;
pub mod tone;

pub use header::{write_header, HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use ops::OP_STREAM_OPEN;
pub use ops::{E_AGAIN, E_INVAL, E_OK};
pub use ops::{OP_CLOSE, OP_FEED_PCM, OP_PAUSE, OP_PLAY_PCM, OP_PLAY_TONE, OP_RESUME, OP_STOP};
pub use tone::{tone_request, TONE_MSG_LEN, TONE_PAYLOAD_LEN};
