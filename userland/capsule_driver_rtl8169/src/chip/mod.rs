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

//! Which Realtek MAC sits behind the BAR. Every later step that differs per
//! revision asks the `MacVersion` found here, numbered as Linux numbers
//! `RTL_GIGA_MAC_VER_<nn>` (r8169.h), so a step can be checked against the
//! Linux function it copies by the same version ranges.

mod detect;
mod entry;
mod error;
mod extended;
mod gmii;
mod info;
mod say;
mod table_fast;
mod table_giga;
mod version;
mod xid;

pub use detect::detect;
pub use error::ChipError;
pub use extended::lookup_extended;
pub use gmii::has_gmii;
pub use info::Chip;
pub use version::MacVersion;
pub use xid::{lookup, xid_of, Lookup};
