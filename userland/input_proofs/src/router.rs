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

//! The input router under test, at the crate root by `pub use router::*`
//! in lib.rs, so its files find each other by the `crate::` paths they use;
//! its `route` tree is mounted in lib.rs itself, as its items are visible
//! `pub(in crate::route)`. A `#[path]` here is relative to src/.

// The input router's press grab: the frame a dragged window's motion arrives in.
pub use state::press as router_press;

// The input router's request wire: the header decode every frame in its inbox
// passes before dispatch, and the encoders a refusal is answered with.
#[path = "router_protocol.rs"]
pub mod router_protocol;

// The router's reserved chord, which no window is handed.
pub use crate::route::chord as router_chord;

/*
 * The input router's routing, whole: its state and route trees under the
 * `crate::` paths they name each other by, its delivery frame, and its peers
 * (window manager, compositor, service lookup) answering from a desk the
 * tests lay out (`clients`). nonos_libc is the shim beside this crate, whose
 * sends land in an outbox the tests read back.
 */
#[path = "../../capsule_input_router/src/state/mod.rs"]
pub mod state;

#[path = "clients/mod.rs"]
pub mod clients;
#[path = "protocol.rs"]
pub mod protocol;

// Which process a key press goes to, by what the window manager answered.
pub use crate::route::key_target as router_key_target;

// The shell's reading of the router's notice frame.
#[path = "../../capsule_desktop_shell/src/state/router_notice.rs"]
pub mod shell_router_notice;
