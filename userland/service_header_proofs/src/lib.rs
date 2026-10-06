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

//! Host proofs for the request header decode of eight services. Any process
//! holding one of their endpoints can send it any bytes, and each decode is
//! the first thing those bytes meet. The protocol modules name their parts by
//! `super::`, so each mounts here under its service's name.

extern crate alloc;

#[path = "../../capsule_attest/src/protocol/mod.rs"]
pub mod attest;
#[path = "../../capsule_desktop_shell/src/protocol/mod.rs"]
pub mod desktop_shell;
#[path = "../../capsule_entropy/src/protocol/mod.rs"]
pub mod entropy;
#[path = "../../capsule_login/src/protocol/mod.rs"]
pub mod login;
#[path = "../../capsule_power/src/protocol/mod.rs"]
pub mod power;
#[path = "../../capsule_driver_ps2_input/src/protocol/mod.rs"]
pub mod ps2_input;
#[path = "../../capsule_driver_virtio_rng/src/protocol/mod.rs"]
pub mod virtio_rng;
#[path = "../../capsule_wallpaper/src/protocol/mod.rs"]
pub mod wallpaper;

#[cfg(test)]
mod harness;
#[cfg(test)]
mod header_tests;
