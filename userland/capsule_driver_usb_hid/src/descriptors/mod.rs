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

mod application;
mod binding;
mod config;
mod packet_size;
mod pad;
mod parse;
mod pointer;
mod types;
mod wire;

pub use binding::HidBinding;
pub use config::{configuration_value, is_hub, total_length, CONFIG_HEADER_LEN};
pub use pad::{pad_layout, PadEvent, PadLayout, PadState};
pub use parse::hid_bindings;
pub use pointer::is_pointer;
pub use types::HidKind;
pub use wire::write_binding;
