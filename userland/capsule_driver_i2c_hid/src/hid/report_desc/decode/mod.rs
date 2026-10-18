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

//! Decode one input report through a parsed layout: a touch sample or a
//! relative mouse sample.

mod body;
mod decode_mouse;
mod decode_touch;
mod position;
mod types;

pub use decode_mouse::decode_mouse;
pub use decode_touch::decode_touch;
pub use types::TouchSample;
// The multitouch path reads its fingers as the touch decode reads its one.
pub(super) use body::holds;
pub(super) use position::position;
