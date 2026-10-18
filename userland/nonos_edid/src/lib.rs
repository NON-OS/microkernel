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

//! A monitor's identification data, read without allocating. `Edid::parse`
//! checks the base block; the extensions are checked one by one as they
//! are walked, and a bad one is skipped rather than failing the panel.

#![no_std]

pub mod base;
pub mod blocks;
pub mod cta;
pub mod displayid;
pub mod edid;
pub mod error;
pub mod query;
pub mod source;
pub mod timing;
pub mod walk;

pub use base::descriptor::Descriptor;
pub use base::dtd::Detailed;
pub use base::range::RangeLimits;
pub use base::size::Native;
pub use base::text::Text;
pub use cta::audio::ShortAudio;
pub use cta::vic::{vic, Vic};
pub use displayid::tiled::Tile;
pub use edid::Edid;
pub use error::Error;
pub use source::Source;
pub use timing::Timing;
