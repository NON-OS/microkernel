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

//! The display wire, NVGP v1 (PLAN-coverage section 12). Every display
//! driver serves it and the compositor drives it; both encode and check
//! frames through this crate so they cannot disagree on a byte.

#![no_std]

pub mod body;
pub mod bytes;
pub mod errno;
pub mod fit;
pub mod header;
pub mod kind;
pub mod notice;
pub mod ops;
pub mod reply;
pub mod request;
pub mod serial;

pub use body::caps::{Caps, CAPS_LEN};
pub use body::cursor::Cursor;
pub use body::flush::Flush;
pub use body::mode::Mode;
pub use body::modes::Modes;
pub use body::rect::Rect;
pub use body::state::State;
pub use body::surface::Surface;
pub use errno::Errno;
pub use fit::fit;
pub use header::{Header, HDR_LEN, MAGIC, MAX_REPLY_BODY, MAX_REQUEST, VERSION};
pub use reply::{reply, Reply};
pub use request::Request;
