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

//! Test 5 on the machine: the shield vectors capsule proves the four pinned
//! production wallet vectors with the prover nonos.shield links, and says
//! on the serial line whether every byte matched, how long each took and
//! the memory it took. Development images only.

#[cfg(feature = "nonos-release")]
compile_error!("the shield vectors capsule is a development test and never ships in a release");

mod embed;
mod spawn;

pub use spawn::spawn_shield_vectors_capsule;
