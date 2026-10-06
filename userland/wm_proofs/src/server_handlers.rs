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

//! `crate::server::handlers`, as far as the body decoders need it: the field
//! reader the handlers share, and window_open's decode, which the dispatch
//! hands every OP_WINDOW_OPEN body to before anything else reads it.

#[path = "../../capsule_wm/src/server/handlers/u32_at.rs"]
pub(crate) mod u32_at;

#[cfg_attr(not(test), allow(dead_code))]
#[path = "../../capsule_wm/src/server/handlers/window_open/decode.rs"]
mod window_open_decode;

#[cfg(test)]
#[path = "window_open_tests.rs"]
mod window_open_tests;

// Where a new window opens: the bar and dock it keeps clear of, and the
// cascade inside the work area between them.
#[cfg(test)]
#[path = "../../capsule_wm/src/server/handlers/window_open/constants.rs"]
mod constants;

#[cfg(test)]
#[path = "../../capsule_wm/src/server/handlers/window_open/cascade.rs"]
mod cascade;

#[cfg(test)]
#[path = "cascade_tests.rs"]
mod cascade_tests;

#[cfg(test)]
#[path = "menubar_tests.rs"]
mod menubar_tests;
