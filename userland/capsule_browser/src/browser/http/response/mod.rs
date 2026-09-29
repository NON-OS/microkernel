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

mod block;
mod bytes;
pub mod charset;
mod codings;
mod content_length;
mod decode_body;
mod fields;
mod frame_len;
mod has_headers;
mod head;
mod head_fields;
mod head_types;
mod is_complete;
mod kind;
mod location;
mod mime;
mod parse;
mod status_code;
mod types;
mod undo_coding;

pub use frame_len::frame_len;
pub use has_headers::{has_headers, wants_close};
pub use is_complete::is_complete;
pub use parse::parse;
pub use types::{ContentKind, Response};
