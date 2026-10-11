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

//! The PCM sink wire the audio server speaks to driver.hda0, served here
//! unchanged: the same magic, header and ops, so the server drives either
//! driver with the same code.

mod codec;
mod header;
mod ops;
mod output;

pub use codec::{parse, refused, response_header, write_status};
pub use header::{Request, HDR_LEN, MAX_PCM_CHUNK, STATUS_LEN};
pub use ops::*;
pub use output::{write_output_status, OUTPUT_STATUS_LEN};
