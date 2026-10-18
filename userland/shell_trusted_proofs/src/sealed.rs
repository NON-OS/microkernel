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

//! What the types hold, checked by the compiler. Each `compile_fail` below
//! is paired with code that compiles, so it fails for the reason named and
//! not for a wrong path.
//!
//! Only `PowerOff::answer` makes the permission to send, and only from the
//! router's token; neither can be built anywhere else.
//! ```
//! use shell_trusted_proofs::state::input_source_rule::{Routed, Router};
//! use shell_trusted_proofs::state::power_off::{Confirmed, PowerOff};
//! fn sends(_: Confirmed) {}
//! let by: Routed = Router::Pinned(1).frame(1).unwrap();
//! let mut p = PowerOff::default();
//! p.ask();
//! sends(p.answer(by, true).unwrap());
//! ```
//! ```compile_fail
//! use shell_trusted_proofs::state::power_off::Confirmed;
//! let _ = Confirmed(());
//! ```
//! ```compile_fail
//! use shell_trusted_proofs::state::input_source_rule::Routed;
//! let _ = Routed(());
//! ```
//! The permission is used up by the one send it is for.
//! ```compile_fail
//! use shell_trusted_proofs::state::input_source_rule::Router;
//! use shell_trusted_proofs::state::power_off::{Confirmed, PowerOff};
//! fn sends(_: Confirmed) {}
//! let mut p = PowerOff::default();
//! p.ask();
//! let go = p.answer(Router::Pinned(1).frame(1).unwrap(), true).unwrap();
//! sends(go.clone());
//! sends(go);
//! ```
//! ```compile_fail
//! use shell_trusted_proofs::state::input_source_rule::Router;
//! use shell_trusted_proofs::state::power_off::{Confirmed, PowerOff};
//! fn sends(_: Confirmed) {}
//! let mut p = PowerOff::default();
//! p.ask();
//! let go = p.answer(Router::Pinned(1).frame(1).unwrap(), true).unwrap();
//! sends(go);
//! sends(go);
//! ```
